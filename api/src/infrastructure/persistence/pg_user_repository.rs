use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

use crate::domain::{
    authentication::model::Provider,
    id::Id,
    user::{
        model::{UpdateUserPayload, User, UserAuthDetail},
        repository::UserRepository,
    },
};

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error> {
        let sql = r#"
            SELECT * FROM users WHERE id = $1;
        "#;
        sqlx::query_as::<_, UserRow>(sql)
            .bind(user_id.get())
            .fetch_optional(&self.pool)
            .await?
            .map(User::try_from)
            .transpose()
    }
    async fn find_user_with_auth_status(
        &self,
        user_id: Id<User>,
    ) -> Result<UserAuthDetail, anyhow::Error> {
        let sql = r#"
            SELECT 
                u.id,
                u.display_name,
                u.email,
                u.avatar_key,
                u.created_at,
                u.updated_at,
                COALESCE(array_agg(a.provider) FILTER (WHERE a.provider IS NOT NULL), '{}') as linked_providers
            FROM users as u 
            LEFT JOIN authentication as a ON u.id = a.user_id
            WHERE u.id = $1
            GROUP BY u.id
        "#;

        let user_auth_detail = sqlx::query_as::<_, UserAuthDetailRow>(sql)
            .bind(user_id.get())
            .fetch_one(&self.pool)
            .await?
            .try_into()?;

        Ok(user_auth_detail)
    }
    async fn update(
        &self,
        user_id: Id<User>,
        update_user: UpdateUserPayload,
    ) -> Result<User, anyhow::Error> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut separeted = query_builder.separated(", ");

        {}
        if let Some(display_name) = update_user.display_name {
            separeted.push("display_name = ");
            separeted.push_bind_unseparated(display_name);
        };

        if let Some(avatar_key) = update_user.avatar_key {
            separeted.push("avatar_key = ");
            separeted.push_bind_unseparated(avatar_key);
        }

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(user_id.get());
        query_builder
            .push(" RETURNING id, display_name, email, avatar_key, role, created_at, updated_at");

        let user = query_builder
            .build_query_as::<UserRow>()
            .fetch_one(&self.pool)
            .await?
            .try_into()?;

        Ok(user)
    }
}

#[derive(Debug, FromRow)]
pub struct UserRow {
    id: i64,
    display_name: String,
    email: String,
    avatar_key: Option<String>,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            display_name: value.display_name,
            email: value.email,
            avatar_key: value.avatar_key,
            role: value.role.parse()?,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}

#[derive(Debug, FromRow)]
pub struct UserAuthDetailRow {
    id: i64,
    display_name: String,
    email: String,
    avatar_key: Option<String>,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    linked_providers: Vec<String>,
}

impl TryFrom<UserAuthDetailRow> for UserAuthDetail {
    type Error = anyhow::Error;

    fn try_from(value: UserAuthDetailRow) -> Result<Self, Self::Error> {
        Ok(Self {
            user: {
                User {
                    id: Id::new(value.id),
                    display_name: value.display_name,
                    email: value.email,
                    avatar_key: value.avatar_key,
                    role: value.role.parse()?,
                    created_at: value.created_at,
                    updated_at: value.updated_at,
                }
            },
            linked_providers: {
                value
                    .linked_providers
                    .iter()
                    .map(|s| s.parse())
                    .collect::<Result<Vec<Provider>, _>>()?
            },
        })
    }
}
