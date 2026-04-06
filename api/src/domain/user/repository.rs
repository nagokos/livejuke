use async_trait::async_trait;

use crate::domain::{id::Id, user::model::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error>;
}
