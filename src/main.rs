pub use self::env::EnvVars;
pub use self::error::{Error, Result};
pub use self::log::log_request;
use crate::error::BootError;
use crate::lottery::service::DynLotteryService;
use axum::{middleware, Router};
use dotenv::dotenv;
use error::BootResult;
use lottery::service::LotteryService;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::net::SocketAddr;
use std::sync::Arc;
mod cron;
mod env;
mod error;
mod health;
mod log;
mod lottery;
mod middlewares;
#[cfg(test)]
use mockall::automock;
struct AppState {
    db: Pool<Postgres>,
    env: EnvVars,
    lottery_service: DynLotteryService,
}

#[cfg_attr(test, automock)]
pub trait AppStateTrait {
    fn get_db(&self) -> Pool<Postgres>;
    fn get_env(&self) -> EnvVars;
    fn get_lottery_service(&self) -> DynLotteryService;
}

impl AppStateTrait for AppState {
    fn get_db(&self) -> Pool<Postgres> {
        self.db.clone()
    }
    fn get_env(&self) -> EnvVars {
        self.env.clone()
    }

    fn get_lottery_service(&self) -> DynLotteryService {
        self.lottery_service.clone()
    }
}

type DynAppState = Arc<dyn AppStateTrait + Send + Sync>;

async fn create_app_state() -> DynAppState {
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
    let envs = EnvVars::new();
    let service = Arc::new(LotteryService {}) as DynLotteryService;
    Arc::new(AppState {
        db: pool,
        env: envs,
        lottery_service: service,
    }) as DynAppState
}
#[tokio::main]
async fn main() -> BootResult {
    let app_state = create_app_state().await;

    let ip_addr = app_state.get_env().server.address;
    let port = app_state.get_env().server.port;
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
