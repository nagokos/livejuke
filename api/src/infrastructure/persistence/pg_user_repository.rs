use chrono::{DateTime, Utc};
use sqlx::{PgPool, prelude::FromRow};

use crate::domain::{
    id::Id,
    user::{model::User, repository::UserRepository},
};

#[derive(Debug, FromRow)]
pub struct UserRow {
    id: i64,
    display_name: String,
    role: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepository {
    async fn find_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error> {
        let sql = r#"
            SELECT * FROM users WHERE id = $1;
        "#;
        let user = sqlx::query_as::<_, UserRow>(sql)
            .bind(user_id.get())
            .fetch_optional(&self.pool)
            .await?
            .map(User::try_from)
            .transpose()?;
        Ok(user)
    }
}

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            display_name: value.display_name,
            role: value.role.parse()?,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
