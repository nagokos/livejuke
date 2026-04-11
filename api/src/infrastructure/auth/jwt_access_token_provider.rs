use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::{
    application::traits::{
        access_token_provider::AccessTokenProvider,
        types::{AccessToken, CurrentUser},
    },
    domain::{
        id::Id,
        user::model::{Role, User},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub iat: u64,
    pub exp: u64,
}

impl Claims {
    fn new(sub: String, role: Role, iat: u64, exp: u64) -> Self {
        Self {
            sub,
            role: role.as_str().to_string(),
            iat,
            exp,
        }
    }
}

#[derive(Clone)]
pub struct JwtAccessTokenProvider {
    jwt_secret: String,
    jwt_exp: u64,
}

impl JwtAccessTokenProvider {
    pub fn new(jwt_secret: String, jwt_exp: u64) -> Self {
        Self {
            jwt_secret,
            jwt_exp,
        }
    }
}

impl AccessTokenProvider for JwtAccessTokenProvider {
    fn generate(&self, sub: Id<User>, role: Role) -> Result<AccessToken, anyhow::Error> {
        let header = Header::new(Algorithm::HS256);

        let now = Utc::now().timestamp() as u64;
        let claims = Claims::new(sub.get().to_string(), role, now, now + self.jwt_exp);
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(AccessToken::new(token))
    }
    fn verify(&self, token: &str) -> Result<CurrentUser, anyhow::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;
        Ok(CurrentUser {
            id: Id::new(token_data.claims.sub.parse()?),
            role: token_data.claims.role.as_str().parse()?,
        })
    }
}
