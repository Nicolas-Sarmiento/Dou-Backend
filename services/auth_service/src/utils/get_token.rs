use axum::{
    extract::FromRequestParts,
    http::{request::Parts, header::AUTHORIZATION, StatusCode},
};

// Wrapper para el token Bearer
pub struct BearerToken(pub String);

impl<S> FromRequestParts<S> for BearerToken
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(AUTHORIZATION).ok_or(StatusCode::UNAUTHORIZED)?;
        let auth_str = auth_header.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

        if let Some(token) = auth_str.strip_prefix("Bearer ") {
            Ok(BearerToken(token.to_string()))
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}
