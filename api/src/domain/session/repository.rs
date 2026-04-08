use async_trait::async_trait;

use crate::domain::{
    id::Id,
    session::model::{NewSession, Session},
    user::model::User,
};

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, new_session: NewSession) -> Result<Session, anyhow::Error>;
    async fn find_by_hash(&self, token_hash: &str) -> Result<Option<Session>, anyhow::Error>;
    async fn revoke(&self, token_hash: &str) -> Result<(), anyhow::Error>;
    async fn revoke_all_by_user_id(&self, user_id: Id<User>) -> Result<(), anyhow::Error>;
}
