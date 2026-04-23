use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

use crate::{
    domain::{
        authentication::{
            model::{Authentication, AuthenticationPayload, Provider},
            repository::AuthRepository,
        },
        id::Id,
        user::model::{User, UserPayload},
    },
    infrastructure::persistence::pg_user_repository::UserRow,
};

pub struct PgAuthenticationRepository {
    pool: PgPool,
}

impl PgAuthenticationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuthRepository for PgAuthenticationRepository {
    async fn create_user_with_authentication(
        &self,
        user: UserPayload,
        authentication: AuthenticationPayload,
    ) -> Result<User, anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let sql = r#"
            INSERT INTO users (
                email
            )
            VALUES ($1)
            RETURNING 
                id,
                display_name,
                email,
                avatar_key,
                role,
                created_at,
                updated_at
        "#;
        let user: User = sqlx::query_as::<_, UserRow>(sql)
            .bind(&user.email)
            .fetch_one(&mut *tx)
            .await?
            .try_into()?;

        let sql = r#"
            INSERT INTO authentications (
                user_id, 
                provider, 
                uid
            )
            VALUES ($1, $2, $3)
        "#;
        sqlx::query(sql)
            .bind(user.id.get())
            .bind(authentication.provider.as_str())
            .bind(&authentication.uid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(user)
    }
    async fn update_user_with_authentication(
        &self,
        user_id: Id<User>,
        authentication: AuthenticationPayload,
    ) -> Result<User, anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let sql = r#"
            UPDATE users 
            SET 
                email = $1
            WHERE
                id = $2
            RETURNING 
                id,
                display_name,
                email,
                avatar_key,
                role,
                created_at,
                updated_at
        "#;
        let user: User = sqlx::query_as::<_, UserRow>(sql)
            .bind(&authentication.uid)
            .bind(user_id.get())
            .fetch_one(&mut *tx)
            .await?
            .try_into()?;

        let sql = r#"
            INSERT INTO authentications (
                user_id, 
                provider, 
                uid 
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, provider)
            DO UPDATE SET uid = EXCLUDED.uid;
        "#;
        sqlx::query(sql)
            .bind(user.id.get())
            .bind(authentication.provider.as_str())
            .bind(&authentication.uid)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(user)
    }
    async fn find_by_provider_uid(
        &self,
        provider: Provider,
        uid: &str,
    ) -> Result<Option<Authentication>, anyhow::Error> {
        let sql = r#"
                SELECT * FROM authentications WHERE provider = $1 AND uid = $2;
            "#;
        sqlx::query_as::<_, AuthenticationRow>(sql)
            .bind(provider.as_str())
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?
            .map(Authentication::try_from)
            .transpose()
    }
}

#[derive(Debug, FromRow)]
pub struct AuthenticationRow {
    id: i64,
    user_id: i64,
    provider: String,
    uid: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<AuthenticationRow> for Authentication {
    type Error = anyhow::Error;

    fn try_from(value: AuthenticationRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            user_id: Id::new(value.user_id),
            provider: value.provider.parse()?,
            uid: value.uid,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
