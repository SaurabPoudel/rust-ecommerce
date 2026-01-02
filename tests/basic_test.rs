use rust_ecommerce::{
    db::users::{create_user, find_by_username, CreateUser},
    config::Config,
};
use bcrypt::verify;
use sqlx::postgres::PgPoolOptions;

#[tokio::test]
async fn test_user_creation_and_authentication() {
    dotenvy::dotenv().ok();
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

    // Clean up if user already exists
    let _ = sqlx::query("DELETE FROM users WHERE username = $1 OR email = $2")
        .bind(&test_user.username)
        .bind(&test_user.email)
        .execute(&db_pool)
        .await;

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
