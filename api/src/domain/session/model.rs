use chrono::{DateTime, Utc};

use crate::domain::{id::Id, user::model::User};

#[derive(Debug)]
pub struct Session {
    pub id: Id<Session>,
    pub user_id: Id<User>,
    pub token_hash: String,
    pub device_info: DeviceInfo,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct NewSession {
    pub user_id: Id<User>,
    pub token_hash: String,
    pub device_info: DeviceInfo,
    pub expires_at: DateTime<Utc>,
}

impl NewSession {
    pub fn new(
        user_id: Id<User>,
        token_hash: String,
        device_info: DeviceInfo,
        expires_at: DateTime<Utc>,
    ) -> Self {
        Self {
            user_id,
            token_hash,
            device_info,
            expires_at,
        }
    }
}

#[derive(Debug)]
pub struct DeviceInfo {
    pub device_name: Option<String>,
    pub model_name: Option<String>,
    pub os: String,
}

impl DeviceInfo {
    pub fn new(device_name: Option<String>, model_name: Option<String>, os: String) -> Self {
        Self {
            device_name,
            model_name,
            os,
        }
    }
}
