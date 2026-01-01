mod api;
mod domain;
mod routes;
mod db;
mod app_state;

use crate::{
    app_state::AppState, 
    db::create_db_pool, 
    routes::create_router
};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_ecommerce=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_pool = create_db_pool().await;
    let app_state = AppState { db_pool };

    let app = create_router(app_state);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}