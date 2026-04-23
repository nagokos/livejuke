use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::domain::{
    authentication::{email::Email, model::Provider},
    id::Id,
    user::display_name::DisplayName,
};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id<User>,
    pub display_name: String,
    pub email: String,
    pub avatar_key: Option<String>,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum Role {
    User,
    Admin,
}

impl Role {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user",
            Self::Admin => "admin",
        }
    }
}

impl FromStr for Role {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "user" => Ok(Self::User),
            "admin" => Ok(Self::Admin),
            _ => Err(anyhow::anyhow!("unknown role: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct UserPayload {
    pub email: String,
}

impl UserPayload {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserAuthDetail {
    pub user: User,
    pub linked_providers: Vec<Provider>,
}

#[derive(Debug, Default)]
pub struct UpdateUserPayload {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub avatar_key: Option<String>,
}

impl UpdateUserPayload {
    pub fn is_empty(&self) -> bool {
        self.display_name.is_none() && self.email.is_none() && self.avatar_key.is_none()
    }
    pub fn display_name(mut self, display_name: DisplayName) -> Self {
        self.display_name = Some(display_name.into_inner());
        self
    }
    pub fn email(mut self, email: Email) -> Self {
        self.email = Some(email.into_inner());
        self
    }
    pub fn avatar_key(mut self, avatar_key: String) -> Self {
        self.avatar_key = Some(avatar_key);
        self
    }
}
