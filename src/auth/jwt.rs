use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey};
use chrono::{Duration, Utc};
use config::Config;
use crate::routes::user::Claims;
use crate::routes::user::BearerToken;
use std::error::Error;

// Define a custom error type for this module
#[derive(Debug)]
pub enum JwtError {
    ConfigError(String),
    TimestampError,
    JwtError(jsonwebtoken::errors::Error),
}

impl From<jsonwebtoken::errors::Error> for JwtError {
    fn from(err: jsonwebtoken::errors::Error) -> JwtError {
        JwtError::JwtError(err)
    }
}

impl Error for JwtError {}

impl std::fmt::Display for JwtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JwtError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            JwtError::TimestampError => write!(f, "Invalid timestamp"),
            JwtError::JwtError(err) => write!(f, "JWT error: {}", err),
        }
    }
}

pub fn generate_token(settings: &Config, user_id: &str) -> Result<String, JwtError> {
    let secret_key = settings.get::<String>("secrets.jwt_secret_key").map_err(|_| JwtError::ConfigError("JWT secret key not found.".to_string()))?;
    let expiration = Utc::now().checked_add_signed(Duration::minutes(60)).ok_or(JwtError::TimestampError)?.timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).map_err(JwtError::from)
}

pub fn validate_token(settings: &Config, token: &BearerToken) -> Result<Claims, JwtError> {
    let secret_key = settings.get::<String>("secrets.jwt_secret_key").map_err(|_| JwtError::ConfigError("JWT secret key not found.".to_string()))?;
    let validation = jsonwebtoken::Validation {
        leeway: 0,
        validate_exp: true,
        algorithms: vec![Algorithm::HS256],
        ..Default::default()
    };

    decode::<Claims>(&token.0, &DecodingKey::from_secret(secret_key.as_ref()), &validation)
        .map(|data| data.claims)
        .map_err(JwtError::from)
}
