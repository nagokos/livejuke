use async_trait::async_trait;

use crate::domain::{id::Id, user::model::User};

#[async_trait]
pub trait UploadSessionStore: Send + Sync {
    async fn set_pending_upload<'a>(
        &'a self,
        key: Id<User>,
        value: &'a str,
    ) -> Result<(), anyhow::Error>;
    async fn get_pending_upload(&self, key: Id<User>) -> Result<String, anyhow::Error>;
}
