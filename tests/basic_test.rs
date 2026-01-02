use rust_ecommerce::{
    app_state::AppState,
    config::Config,
    db::users::{create_user, find_by_username, CreateUser},
};
use bcrypt::verify;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_user_creation_and_authentication() {
    // Set up test database connection
    let config = Config::from_env().expect("Failed to load test config");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create test database pool");

    // Test user data
    let test_user = CreateUser {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        password: "testpassword123".to_string(),
    };

    // Test user creation
    let created_user = create_user(&db_pool, test_user.clone())
        .await
        .expect("Failed to create test user");

    assert_eq!(created_user.username, test_user.username);
    assert_eq!(created_user.email, test_user.email);

    // Test password hashing
    let stored_user = find_by_username(&db_pool, &test_user.username)
        .await
        .expect("Failed to find test user")
        .expect("Test user not found");

    // Verify the password hash
    let is_valid = verify(&test_user.password, &stored_user.password_hash)
        .expect("Failed to verify password hash");
    
    assert!(is_valid, "Password verification failed");
}

#[tokio::test]
async fn test_user_creation_duplicate_username() {
    // Set up test database connection
    let config = Config::from_env().expect("Failed to load test config");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to create test database pool");

    // Test user data
    let test_user = CreateUser {
        username: "duplicate_user".to_string(),
        email: "test1@example.com".to_string(),
        password: "testpassword123".to_string(),
    };

    // Create first user (should succeed)
    create_user(&db_pool, test_user.clone())
        .await
        .expect("Failed to create first test user");

    // Try to create user with same username (should fail)
    let result = create_user(&db_pool, test_user).await;
    assert!(result.is_err(), "Should not allow creating user with duplicate username");
}
