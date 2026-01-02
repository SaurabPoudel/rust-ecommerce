use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use rust_ecommerce::{
    app_state::AppState,
    config::Config,
    db::users::CreateUser,
    router,
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower::ServiceExt;
use tower_http::ServiceBuilderExt;

async fn setup_test_app() -> Router {
    // Load test configuration
    let config = Config::from_env().expect("Failed to load test config");
    
    // Create a test database connection pool
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create test database pool");

    // Create app state
    let app_state = AppState {
        db_pool: db_pool.clone(),
        config: Arc::new(config),
    };

    // Set up the application router
    router::create_router(app_state)
}

#[tokio::test]
async fn test_register_and_login() {
    // Set up test environment
    std::env::set_var("JWT_SECRET", "test_secret");
    std::env::set_var("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/rust_ecommerce_test");
    
    let app = setup_test_app().await;

    // Test user data
    let test_user = CreateUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "testpassword123".to_string(),
    };

    // Test registration
    let register_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/users/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&test_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(register_response.status(), StatusCode::OK);

    // Test login with correct credentials
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/users/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "password": "testpassword123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(login_response.status(), StatusCode::OK);

    // Extract token from login response
    let body = hyper::body::to_bytes(login_response.into_body()).await.unwrap();
    let token: String = serde_json::from_slice(&body).unwrap();
    assert!(!token.is_empty());

    // Test accessing protected route with valid token
    let protected_response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/api/users/me")
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(protected_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_invalid_credentials() {
    let app = setup_test_app().await;

    // Test login with non-existent user
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/api/users/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "nonexistent",
                        "password": "wrongpassword"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(login_response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_route_without_token() {
    let app = setup_test_app().await;

    // Try to access protected route without token
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/api/users/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
