use rand::distr::{Alphanumeric, SampleString};
use serde::{Deserialize, Serialize};

use crate::domain::{
    id::Id,
    user::model::{Role, User},
};

#[derive(Debug)]
pub struct RefreshToken(String);

impl RefreshToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RefreshToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct AccessToken(String);

impl AccessToken {
    pub fn new(value: String) -> Self {
        Self(value)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub struct ExternalUserInfo {
    pub sub: String,
    pub email: String,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Id<User>,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationData {
    pub code: String,
}

impl VerificationData {
    pub fn new() -> Self {
        let code = Alphanumeric.sample_string(&mut rand::rng(), 6);
        Self { code }
    }
}
