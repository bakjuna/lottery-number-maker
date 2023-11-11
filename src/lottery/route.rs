use axum::{Router, routing::get};

pub fn router_lottery() -> Router {
    Router::new().route("/", get(super::handler::handler_lottery))
}
