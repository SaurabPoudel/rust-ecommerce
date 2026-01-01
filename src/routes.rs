use crate::{
    api::{health_check::health_check, users::{register, login, me}},
    app_state::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/users/register", post(register))
        .route("/users/login", post(login))
        .route("/users/me", get(me))
        .with_state(app_state)
}
