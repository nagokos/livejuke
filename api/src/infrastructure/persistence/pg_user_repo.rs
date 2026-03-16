use sqlx::PgPool;

use crate::domain::{models::user::User, repositories::user_repository::UserRepository};

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for PgUserRepo {
    fn create_user(&self) -> anyhow::Result<User> {
        todo!()
    }
}
