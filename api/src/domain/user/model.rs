use std::str::FromStr;

use chrono::{DateTime, Utc};

use crate::domain::id::Id;

#[derive(Debug)]
pub struct User {
    pub id: Id<User>,
    pub display_name: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum Role {
    User,
    Admin,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Admin => write!(f, "admin"),
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
    pub display_name: String,
}
