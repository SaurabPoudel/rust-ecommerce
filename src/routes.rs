use crate::{
    api::{
        health_check::health_check, 
        users::{register, login, me},
        products::{list_products_handler, get_product_handler, create_product_handler},
        orders::{create_order_handler, list_my_orders_handler, get_order_handler},
    },
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
        .route("/products", get(list_products_handler).post(create_product_handler))
        .route("/products/{id}", get(get_product_handler))
        .route("/orders", get(list_my_orders_handler).post(create_order_handler))
        .route("/orders/{id}", get(get_order_handler))
        .with_state(app_state)
}
