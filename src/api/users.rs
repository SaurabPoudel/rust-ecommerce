use crate::{
    app_state::AppState,
    api::auth::AuthUser,
    db::users::{create_user, find_by_username, CreateUser},
    domain::models::User,
};
use axum::{extract::State, http::StatusCode, Json};
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use tracing::error;

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
}

#[derive(serde::Serialize)]
pub struct UserResponse {
    id: uuid::Uuid,
    username: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<UserResponse>, StatusCode> {
    match create_user(&app_state.db_pool, payload).await {
        Ok(user) => Ok(Json(user.into())),
        Err(err) => {
            error!("Error creating user: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginUser>,
) -> Result<Json<String>, StatusCode> {
    let user = find_by_username(&app_state.db_pool, &payload.username)
        .await
        .map_err(|err| {
            error!("Error finding user: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let is_valid_password = verify(&payload.password, &user.password_hash)
        .map_err(|err| {
            error!("Error verifying password: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !is_valid_password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration,
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
        .map_err(|err| {
            error!("Error encoding token: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(token))
}

pub async fn me(AuthUser(user_id): AuthUser) -> Result<Json<String>, StatusCode> {
    Ok(Json(format!("Authenticated user ID: {}", user_id)))
}
