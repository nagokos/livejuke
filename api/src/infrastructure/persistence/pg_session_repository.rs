use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, prelude::FromRow};

use crate::domain::{
    id::Id,
    session::{
        model::{DeviceInfo, NewSession, Session},
        repository::SessionRepository,
    },
    user::model::User,
};

pub struct PgSessionRepository {
    pool: PgPool,
}

impl PgSessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn create(&self, new_session: NewSession) -> Result<Session, anyhow::Error> {
        let sql = r#"
            INSERT INTO sessions (
                user_id,
                token_hash,
                device_name,
                model_name,
                os,
                expires_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING 
                id, 
                user_id, 
                token_hash, 
                device_name, 
                model_name, 
                os, 
                is_revoked, 
                expires_at, 
                created_at, 
                updated_at
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
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<Session>, anyhow::Error> {
        let sql = r#"
            SELECT * FROM sessions WHERE token_hash = $1;
        "#;

        sqlx::query_as::<_, SessionRow>(sql)
            .bind(token_hash)
            .fetch_optional(&self.pool)
            .await?
            .map(Session::try_from)
            .transpose()
    }
    async fn revoke(&self, token_hash: &str) -> Result<(), anyhow::Error> {
        let sql = r#"
            UPDATE sessions 
            SET 
                is_revoked = true
            WHERE token_hash = $1
        "#;

        sqlx::query(sql)
            .bind(token_hash)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
    async fn revoke_all_by_user_id(&self, user_id: Id<User>) -> Result<(), anyhow::Error> {
        let sql = r#"
            UPDATE sessions 
            SET 
                is_revoked = true
            WHERE user_id = $1
        "#;

        sqlx::query(sql)
            .bind(user_id.get())
            .execute(&self.pool)
            .await?;

        Ok(())
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
