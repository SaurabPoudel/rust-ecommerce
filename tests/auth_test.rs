use axum::{
    body::Body,
    http::{self, Request, StatusCode},
    Router,
};
use rust_ecommerce::{
    app_state::AppState,
    config::Config,
    db::users::CreateUser,
    routes::create_router,
};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use http_body_util::BodyExt;

async fn setup_test_app() -> Router {
    dotenvy::dotenv().ok();
    let config = Config::from_env().expect("Failed to load test config");
    
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create test database pool");

    let app_state = AppState {
        db_pool: db_pool.clone(),
    };

    create_router(app_state)
}

#[tokio::test]
async fn test_register_and_login() {
    let app = setup_test_app().await;

    let username = format!("user_{}", uuid::Uuid::new_v4());
    let email = format!("{}@example.com", username);

    // Test user data
    let test_user = CreateUser {
        username: username.clone(),
        email: email.clone(),
        password: "testpassword123".to_string(),
    };

    // Test registration
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users/register")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(serde_json::to_string(&test_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Test login
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": username,
                        "password": "testpassword123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let token: String = serde_json::from_slice(&body).unwrap();
    assert!(!token.is_empty());

    // Test /users/me
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/users/me")
                .header(http::header::AUTHORIZATION, format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
