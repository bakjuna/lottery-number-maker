use crate::error::BootError;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;

pub use self::env::EnvVars;
pub use self::error::{Error, Result};
pub use self::log::log_request;
use axum::{middleware, Router};
use error::BootResult;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
mod cron;
mod env;
mod error;
mod health;
mod log;
mod lottery;
mod middlewares;

pub struct AppState {
    db: Pool<Postgres>,
    env: EnvVars,
}

async fn create_app_state() -> Arc<AppState> {
    println!("Starting Server...");
    dotenv().ok();
    let envs = EnvVars::new();
    let database_url = format!(
        "{}{}{}{}{}{}{}{}{}{}{}",
        "postgres://",
        envs.postgres.user,
        ":",
        envs.postgres.password,
        "@",
        envs.postgres.host,
        ":",
        envs.postgres.port,
        "/",
        envs.postgres.database,
        "?schema=public"
    );
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("database connected");
            pool
        }
        Err(err) => {
            println!("database not connected: {:?}", err);
            std::process::exit(1);
        }
    };
    Arc::new(AppState {
        db: pool.clone(),
        env: envs,
    })
}
#[tokio::main]
async fn main() -> BootResult {
    let app_state = create_app_state().await;
    let ip_addr = app_state.clone().env.server.address;
    let port = app_state.clone().env.server.port;
    let cron_jobs = cron::creator::create_cron_jobs().await.unwrap();
    cron_jobs.start().await.unwrap();

    let routers_all: Router = Router::new()
        .nest("/", health::route::router_health())
        .nest("/lottery", lottery::route::router_lottery(app_state))
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));


    let addr = SocketAddr::new(ip_addr, port);
    println!("->> LISTENING on {addr} \n");

    let server = axum::Server::bind(&addr)
        .serve(routers_all.into_make_service())
        .await;
    match server {
        Ok(app) => Ok(app),
        Err(err) => match err {
            _ => Err(BootError::ApiFailed),
        },
    }
}
