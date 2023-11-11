use std::net::SocketAddr;
use crate::error::BootError;

pub use self::error::{Error, Result};
pub use self::log::log_request;
use axum::{middleware, Router};
use error::BootResult;
mod error;
mod health;
mod log;
mod lottery;
mod middlewares;
mod cron;

#[tokio::main]
async fn main() -> BootResult {
	let cron_jobs = cron::creator::create_cron_jobs().await.unwrap();
	cron_jobs.start().await.unwrap();
    let routers_all: Router = Router::new()
        .nest("/", health::route::router_health())
        .nest("/lottery", lottery::route::router_lottery())
        .layer(middleware::map_response(
            middlewares::middleware::main_response_mapper,
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr} \n");

    let server = axum::Server::bind(&addr)
        .serve(routers_all.into_make_service())
        .await;
	match server {
		Ok(app) => {
			Ok(app)
		}
		Err(err) => {
			match err {
				_ => {
					Err(BootError::ApiFailed)
				}
			}
		}
	}
}
