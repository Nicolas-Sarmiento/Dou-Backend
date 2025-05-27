use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::env;
use crate::models::Claims;

pub struct AuthenticatedUser(pub Claims);

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    fn from_request_parts<'a>(
        parts: &'a mut Parts,
        _state: &S,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Rejection>> + Send + 'a>> {
        Box::pin(async move {
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok());

            if let Some(auth_header) = auth_header {
                if let Some(token) = auth_header.strip_prefix("Bearer ") {
                    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");

                    let decoded = decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(secret.as_ref()),
                        &Validation::new(Algorithm::HS256),
                    )
                    .map_err(|_| StatusCode::UNAUTHORIZED)?;

                    return Ok(AuthenticatedUser(decoded.claims));
                }
            }

            Err(StatusCode::UNAUTHORIZED)
        })
    }
}
