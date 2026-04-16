use async_trait::async_trait;

use crate::domain::{
    id::Id,
    user::model::{UpdateUser, User},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error>;
    async fn update(
        &self,
        user_id: Id<User>,
        update_user: UpdateUser,
    ) -> Result<User, anyhow::Error>;
}
