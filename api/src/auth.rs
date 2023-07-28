use crate::error::{ApiError, Result};
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use std::env;

pub async fn validate_user(user: String, password: Option<String>) -> Result<()> {
    let e = ApiError::Forbidden("unauthorized".into());
    let auth_user = env::var("AUTH_USER").map_err(|_| e.clone())?;
    let auth_password = env::var("AUTH_PASSWORD").map_err(|_| e.clone())?;
    let password = password.ok_or(e.clone())?;
    if auth_user == user && auth_password == password {
        Ok(())
    } else {
        Err(e)
    }
}

pub async fn validate_token(token: &str) -> Result<String> {
    let jwks = reqwest::get("https://auth.vdslab.jp/.well-known/jwks.json")
        .await
        .map_err(|_| ApiError::Unknown("error".into()))?
        .json::<JWKS>()
        .await
        .map_err(|_| ApiError::Unknown("error".into()))?;

    let validations = vec![
        Validation::Issuer("https://auth.vdslab.jp/".into()),
        Validation::SubjectPresent,
    ];

    let kid = token_kid(&token)
        .expect("Failed to decode token headers")
        .expect("No 'kid' claim present in token");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    let jwt = validate(token, jwk, validations).expect("Token validation has failed!");

    Ok(jwt.claims["sub"].as_str().unwrap().into())
}
