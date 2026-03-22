use std::ops::Add;

use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::{
    application::traits::token_provider::TokenProvider,
    domain::{
        id::Id,
        user::model::{Role, User},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub role: String,
    pub iat: i64,
    pub exp: i64,
}

impl Claims {
    fn new(sub: i64, role: Role, exp: i64) -> Self {
        Self {
            sub,
            role: role.as_str().to_string(),
            iat: Utc::now().timestamp(),
            exp: Utc::now().timestamp().add(exp),
        }
    }
}

pub struct JwtTokenProvider {
    pub jwt_secret: String,
    pub jwt_expiration: i64,
}

impl JwtTokenProvider {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
        }
    }
}

impl TokenProvider for JwtTokenProvider {
    fn generate(&self, sub: Id<User>, role: Role) -> Result<String, anyhow::Error> {
        let header = Header::new(Algorithm::HS256);
        let claims = Claims::new(sub.get(), role, self.jwt_expiration);
        Ok(jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?)
    }
}
