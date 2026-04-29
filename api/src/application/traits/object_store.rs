use async_trait::async_trait;

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn get_presigned_uri(
        &self,
        key: String,
        media_type: &str,
    ) -> Result<String, anyhow::Error>;
    async fn remove_object(&self, key: String) -> Result<(), anyhow::Error>;
}
