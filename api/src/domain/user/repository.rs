use async_trait::async_trait;

use crate::domain::{
    authentication::email::Email,
    id::Id,
    user::model::{UpdateUserPayload, User, UserAuthDetail},
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: Id<User>) -> Result<Option<User>, anyhow::Error>;
    async fn find_user_with_auth_status(
        &self,
        user_id: Id<User>,
    ) -> Result<UserAuthDetail, anyhow::Error>;
    async fn find_by_email(&self, email: Email) -> Result<Option<User>, anyhow::Error>;
    async fn update(
        &self,
        user_id: Id<User>,
        update_user: UpdateUserPayload,
    ) -> Result<User, anyhow::Error>;
}
