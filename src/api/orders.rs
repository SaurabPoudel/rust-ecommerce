use crate::{
    api::auth::AuthUser,
    app_state::AppState,
    db::orders::{create_order, list_orders_by_user, get_order, CreateOrder},
    domain::models::{Order, OrderItem},
};
use axum::{extract::{State, Path}, http::StatusCode, Json};
use serde::Serialize;
use uuid::Uuid;
use tracing::error;

pub async fn create_order_handler(
    State(app_state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateOrder>,
) -> Result<Json<Order>, StatusCode> {
    match create_order(&app_state.db_pool, user_id, payload).await {
        Ok(order) => Ok(Json(order)),
        Err(err) => {
            error!("Error creating order: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn list_my_orders_handler(
    State(app_state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<Vec<Order>>, StatusCode> {
    match list_orders_by_user(&app_state.db_pool, user_id).await {
        Ok(orders) => Ok(Json(orders)),
        Err(err) => {
            error!("Error listing orders: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Serialize)]
pub struct OrderDetailResponse {
    pub order: Order,
    pub items: Vec<OrderItem>,
}

pub async fn get_order_handler(
    State(app_state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(order_id): Path<Uuid>,
) -> Result<Json<OrderDetailResponse>, StatusCode> {
    match get_order(&app_state.db_pool, order_id).await {
        Ok(Some((order, items))) => {
            if order.user_id != user_id {
                return Err(StatusCode::FORBIDDEN);
            }
            Ok(Json(OrderDetailResponse { order, items }))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!("Error getting order: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
