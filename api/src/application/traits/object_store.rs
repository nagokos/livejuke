use async_trait::async_trait;

#[async_trait]
pub trait ObjectStore: Send + Sync {
    async fn get_presigned_uri(&self, key: &str, media_type: &str)
    -> Result<String, anyhow::Error>;
}
