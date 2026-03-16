use sqlx::PgPool;

use crate::domain::repositories::auth_repository::AuthRepository;

pub struct PgAuthRepo {
    pool: PgPool,
}

impl PgAuthRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AuthRepository for PgAuthRepo {
    fn create_authentication(&self) -> anyhow::Result<()> {
        todo!()
    }
}
