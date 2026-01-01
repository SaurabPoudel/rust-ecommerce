use crate::app_state::AppState;
use axum::{extract::State, http::StatusCode};

pub async fn health_check(State(app_state): State<AppState>) -> StatusCode {
    let result = sqlx::query("SELECT 1")
        .execute(&app_state.db_pool)
        .await;

    if result.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
