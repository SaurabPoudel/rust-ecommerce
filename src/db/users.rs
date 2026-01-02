use crate::domain::models::User;
use sqlx::PgPool;
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    db_pool: &PgPool,
    new_user: CreateUser,
) -> Result<User, Box<dyn std::error::Error>> {
    let password_hash = hash(new_user.password, DEFAULT_COST)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, username, email, password_hash)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, email, password_hash, created_at, updated_at
        "#,
        Uuid::new_v4(),
        new_user.username,
        new_user.email,
        password_hash,
    )
    .fetch_one(db_pool)
    .await?;

    Ok(user)
}

pub async fn find_by_username(
    db_pool: &PgPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, created_at, updated_at
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(user)
}
