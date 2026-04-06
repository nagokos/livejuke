use async_trait::async_trait;

use crate::{application::traits::types::VerificationData, domain::authentication::email::Email};

#[async_trait]
pub trait VerificationCodeStore: Send + Sync {
    async fn is_rate_limited(&self, email: &Email) -> Result<bool, anyhow::Error>;
    async fn increment_rate_limit(&self, email: &Email) -> Result<(), anyhow::Error>;
    async fn increment_attempts(&self, email: &Email) -> Result<(), anyhow::Error>;
    async fn is_max_attempts(&self, email: &Email) -> Result<bool, anyhow::Error>;
    async fn save(&self, email: &Email, data: &VerificationData) -> Result<(), anyhow::Error>;
    async fn find(&self, email: &Email) -> Result<Option<VerificationData>, anyhow::Error>;
    async fn delete(&self, email: &Email) -> Result<(), anyhow::Error>;
}
