use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{
    lottery::handler::handler_lottery, AppState
};

pub fn router_lottery(app_state: Arc<AppState>) -> Router {
    Router::new().route("/", get(handler_lottery))
        .with_state(app_state)
}
