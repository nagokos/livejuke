use anyhow::Ok;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

use crate::{
    domain::{
        authentication::{
            error::AuthenticationError,
            model::{Authentication, NewAuthentication, Provider},
            repository::AuthRepository,
        },
        id::Id,
        user::model::{NewUser, User},
    },
    infrastructure::persistence::pg_user_repository::UserRow,
};

#[derive(Debug, FromRow)]
pub struct AuthenticationRow {
    id: i64,
    user_id: i64,
    provider: String,
    uid: String,
    password_digest: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PgAuthenticationRepository {
    pool: PgPool,
}

impl PgAuthenticationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AuthRepository for PgAuthenticationRepository {
    async fn create_user_with_authentication(
        &self,
        new_user: NewUser,
        new_authentication: NewAuthentication,
    ) -> Result<User, anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let sql = r#"
            INSERT INTO users (display_name)
            VALUES ($1)
            RETURNING *
        "#;
        let user: User = sqlx::query_as::<_, UserRow>(sql)
            .bind(new_user.display_name.into_inner())
            .fetch_one(&mut *tx)
            .await?
            .try_into()?;

        let sql = r#"
            INSERT INTO authentications (user_id, provider, uid, password_digest)
            VALUES ($1, $2, $3, $4)
            RETURNING *
        "#;
        let result = sqlx::query(sql)
            .bind(user.id.get())
            .bind(new_authentication.provider.as_str())
            .bind(new_authentication.uid)
            .bind(new_authentication.password_digest)
            .execute(&mut *tx)
            .await;

        if let Err(e) = result {
            match e {
                sqlx::Error::Database(e) if e.code().as_deref() == Some("23505") => {
                    return Err(AuthenticationError::EmailAlreadyExists.into());
                }
                _ => return Err(e.into()),
            }
        }

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
        let authentication = sqlx::query_as::<_, AuthenticationRow>(sql)
            .bind(provider.as_str())
            .bind(uid)
            .fetch_optional(&self.pool)
            .await?
            .map(Authentication::try_from)
            .transpose()?;
        Ok(authentication)
    }
}

impl TryFrom<AuthenticationRow> for Authentication {
    type Error = anyhow::Error;

    fn try_from(value: AuthenticationRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            user_id: Id::new(value.user_id),
            provider: value.provider.parse()?,
            uid: value.uid,
            password_digest: value.password_digest,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
