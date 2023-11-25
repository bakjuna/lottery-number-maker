pub use self::app_state::*;
pub use self::env::EnvVars;
pub use self::errors::{BootError, Error, Result};
pub use self::log::log_request;
pub use self::test::*;
pub use self::lottery::service::DynLotteryService;
use axum::{middleware, Router};
use errors::BootResult;

use sqlx::migrate;
use std::net::{SocketAddr, IpAddr};
use std::sync::Arc;
mod app_state;
mod cron;
mod env;
mod errors;
mod health;
mod log;
mod lottery;
mod middlewares;
mod test;

#[tokio::main]
async fn main() -> BootResult {
    println!("Starting Server...");
    let app_state = create_app_state().await;
    handle_cronjob().await;
    handle_migrations(app_state.clone()).await;
    handle_router(app_state.clone()).await

}

async fn handle_migrations(app_state: Arc<dyn AppStateTrait + Send + Sync>) {
    println!("Migrating Database...");
    let migrations = migrate!();
    let db = app_state.get_db();
    migrations.run(&db).await.unwrap_or_else(|e| {
        println!("Migration Failed: {}", e);
    });
    println!("Migrating Completed");
}

async fn handle_cronjob() {
    println!("Creating Cronjobs...");
    let cron_jobs = cron::creator::create_cron_jobs().await.unwrap();
    println!("Creating Cronjobs Completed");
    cron_jobs.start().await.unwrap();
}

async fn handle_router(app_state: Arc<dyn AppStateTrait + Send + Sync>) -> BootResult {
    println!("Creating Routers...");
    let ip_addr: IpAddr = app_state.get_env().server.address;
    let port: u16 = app_state.get_env().server.port;

    let routers_all: Router = Router::new()
        .nest("/", health::route::router_health())
        .nest("/lottery", lottery::route::router_lottery(app_state))
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));
    let addr: SocketAddr = SocketAddr::new(ip_addr, port);
    println!("->> LISTENING on {addr} \n");
    let server = axum::Server::bind(&addr)
    .serve(routers_all.into_make_service())
    .await;
    match server {
        Ok(app) => Ok(app),
        Err(_err) => Err(BootError::Api),
    }
}