use crate::error::{ApiError, Result};
use alcoholic_jwt::{token_kid, validate, Validation, JWKS};
use async_trait::async_trait;

#[async_trait]
pub trait Validator: Clone + Send + Sync + 'static {
    async fn validate_user(&self, payload: (String, Option<String>)) -> Result<()>;
    async fn validate_token(&self, token: String) -> Result<String>;
}

#[derive(Clone)]
pub struct ValidatorImpl {
    pub user: String,
    pub password: String,
}

#[async_trait]
impl Validator for ValidatorImpl {
    async fn validate_user(&self, payload: (String, Option<String>)) -> Result<()> {
        if self.user == payload.0 && Some(self.password.clone()) == payload.1 {
            Ok(())
        } else {
            Err(ApiError::Forbidden("Login required".into()))
        }
    }

    async fn validate_token(&self, token: String) -> Result<String> {
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
        let jwt = validate(&token, jwk, validations).expect("Token validation has failed!");
        Ok(jwt.claims["sub"].as_str().unwrap().into())
    }
}

#[derive(Clone)]
pub struct ValidatorMock;

impl ValidatorMock {
    pub const VALID_USER: &str = "user";
    pub const VALID_PASSWORD: &str = "password";
    pub const VALID_TOKEN: &str = "validtoken";
    pub const INVALID_TOKEN: &str = "invalidtoken";
}

#[async_trait]
impl Validator for ValidatorMock {
    async fn validate_user(&self, (user, password): (String, Option<String>)) -> Result<()> {
        if let Some(password) = password {
            if user == Self::VALID_USER && password == Self::VALID_PASSWORD {
                return Ok(());
            }
        }
        Err(ApiError::Forbidden("Login required".into()))
    }

    async fn validate_token(&self, token: String) -> Result<String> {
        if token == Self::VALID_TOKEN {
            Ok("user".into())
        } else {
            Err(ApiError::Forbidden("Login required".into()))
        }
    }
}
