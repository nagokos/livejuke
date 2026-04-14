use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::domain::id::Id;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Id<User>,
    pub display_name: String,
    pub email: String,
    pub avatar_key: String,
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
pub struct NewUser {
    pub email: String,
}

impl NewUser {
    pub fn new(email: &str) -> Self {
        Self {
            email: email.to_string(),
        }
    }
}
