use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

use crate::domain::{
    id::Id,
    user::{
        model::{UpdateUser, User},
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
    async fn update(
        &self,
        user_id: Id<User>,
        update_user: UpdateUser,
    ) -> Result<User, anyhow::Error> {
        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut separeted = query_builder.separated(", ");

        if let Some(display_name) = update_user.display_name {
            separeted.push("display_name = ");
            query_builder.push_bind(display_name);
        };

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
