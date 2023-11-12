
use axum::{routing::get, Router};

use crate::{lottery::handler::handler_lottery, DynAppState};

pub fn router_lottery(app_state: DynAppState) -> Router {
    Router::new()
        .route("/", get(handler_lottery))
        .with_state(app_state)
}
