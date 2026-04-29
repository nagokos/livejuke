use async_trait::async_trait;
use aws_config::{Region, meta::region::RegionProviderChain};
use aws_sdk_s3::{Client, presigning::PresigningConfig};

use crate::application::traits::object_store::ObjectStore;

const PRESIGNED_URL_EXPIRES_SECS: u64 = 300;

pub struct AwsS3Store {
    client: Client,
    bucket: String,
}

impl AwsS3Store {
    pub async fn new(bucket: String) -> Self {
        let region_provider =
            RegionProviderChain::default_provider().or_else(Region::new("ap-northeast-1"));
        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);
        Self { client, bucket }
    }
}

#[async_trait]
impl ObjectStore for AwsS3Store {
    async fn get_presigned_uri(
        &self,
        key: String,
        media_type: &str,
    ) -> Result<String, anyhow::Error> {
        let expires_in: std::time::Duration =
            std::time::Duration::from_secs(PRESIGNED_URL_EXPIRES_SECS);
        let expires_in: aws_sdk_s3::presigning::PresigningConfig =
            PresigningConfig::expires_in(expires_in).map_err(|err| {
                anyhow::anyhow!(format!(
                    "Failed to convert expiration to PresigningConfig: {err:?}"
                ))
            })?;

        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .content_type(media_type)
            .key(key)
            .presigned(expires_in)
            .await?;

        Ok(presigned_request.uri().into())
    }
    async fn remove_object(&self, key: String) -> Result<(), anyhow::Error> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;

        Ok(())
    }
}
