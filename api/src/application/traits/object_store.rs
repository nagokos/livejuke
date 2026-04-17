use async_trait::async_trait;

use crate::domain::shared::media_type::MediaType;

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn get_presigned_uri(
        &self,
        key: &str,
        media_type: MediaType,
    ) -> Result<String, anyhow::Error>;
}
