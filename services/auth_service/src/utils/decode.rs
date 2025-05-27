use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use crate::models::user_auth::Claims;
use std::env;

pub fn decode_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not defined!");
    let validation = Validation::new(Algorithm::HS256);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &validation,
    )
}
