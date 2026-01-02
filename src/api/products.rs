use crate::{
    app_state::AppState,
    db::products::{create_product, get_product, list_products, CreateProduct},
    domain::models::Product,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::error;
use uuid::Uuid;

pub async fn list_products_handler(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Product>>, StatusCode> {
    match list_products(&app_state.db_pool).await {
        Ok(products) => Ok(Json(products)),
        Err(err) => {
            error!("Error listing products: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_product_handler(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    match get_product(&app_state.db_pool, id).await {
        Ok(Some(product)) => Ok(Json(product)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!("Error getting product: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_product_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<Json<Product>, StatusCode> {
    match create_product(&app_state.db_pool, payload).await {
        Ok(product) => Ok(Json(product)),
        Err(err) => {
            error!("Error creating product: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
