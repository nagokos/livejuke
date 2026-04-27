use async_trait::async_trait;

use crate::application::traits::types::ExternalUserInfo;

#[derive(Debug, thiserror::Error)]
pub enum OidcVerifyError {
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Email not verified")]
    EmailNotVerified,
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("Failed to parse JWK: {0}")]
    Parse(#[from] serde_json::Error),
    #[error("Internal error: {0}")]
    Internal(String),
}

#[async_trait]
pub trait IdTokenVerifier: Send + Sync {
    async fn verify(&self, id_token: &str) -> Result<ExternalUserInfo, OidcVerifyError>;
}
