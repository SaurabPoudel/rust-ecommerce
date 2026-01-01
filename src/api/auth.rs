use crate::api::users::Claims;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{extract::TypedHeader, headers::{authorization::Bearer, Authorization}};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use tracing::error;

pub struct AuthUser(pub uuid::Uuid);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {


            let TypedHeader(Authorization(bearer)) = parts.extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        let jwt_secret = env::var("JWT_SECRET").map_err(|err| {
            error!("JWT_SECRET not set: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|err| {
            error!("JWT validation error: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

        let user_id = uuid::Uuid::parse_str(&token_data.claims.sub).map_err(|err| {
            error!("Invalid UUID in JWT claims: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

        Ok(AuthUser(user_id))
    }
}
