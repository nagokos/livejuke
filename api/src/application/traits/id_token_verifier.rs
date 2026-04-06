use async_trait::async_trait;

use crate::application::traits::types::ExternalUserInfo;

#[async_trait]
pub trait IdTokenVerifier: Send + Sync {
    async fn verify(&self, id_token: &str) -> Result<ExternalUserInfo, anyhow::Error>;
}
