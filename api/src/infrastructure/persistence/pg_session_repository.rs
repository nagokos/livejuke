use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::domain::{
    id::Id,
    session::{
        model::{DeviceInfo, NewSession, Session},
        repository::SessionRepository,
    },
};

pub struct PgSessionRepository {
    pool: PgPool,
}

impl PgSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SessionRepository for PgSessionRepository {
    async fn create(&self, new_session: NewSession) -> Result<Session, anyhow::Error> {
        let sql = r#"
            INSERT INTO refresh_tokens (user_id, token_hash, device_name, model_name, os, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
        "#;

        let session = sqlx::query_as::<_, SessionRow>(sql)
            .bind(new_session.user_id.get())
            .bind(new_session.token_hash)
            .bind(new_session.device_info.device_name)
            .bind(new_session.device_info.model_name)
            .bind(new_session.device_info.os)
            .bind(new_session.expires_at)
            .fetch_one(&self.pool)
            .await?
            .try_into()?;

        Ok(session)
    }
}

#[derive(Serialize, Debug, FromRow)]
pub struct SessionRow {
    pub id: i64,
    pub user_id: i64,
    pub token_hash: String,
    pub device_name: Option<String>,
    pub model_name: Option<String>,
    pub os: String,
    pub is_revoked: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<SessionRow> for Session {
    type Error = anyhow::Error;

    fn try_from(value: SessionRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            user_id: Id::new(value.user_id),
            token_hash: value.token_hash,
            device_info: DeviceInfo {
                device_name: value.device_name,
                model_name: value.model_name,
                os: value.os,
            },
            is_revoked: value.is_revoked,
            expires_at: value.expires_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
