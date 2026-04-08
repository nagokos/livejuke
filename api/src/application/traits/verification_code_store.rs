use async_trait::async_trait;

use crate::{application::traits::types::VerificationData, domain::authentication::email::Email};

#[async_trait]
pub trait VerificationCodeStore: Send + Sync {
    fn is_rate_limited(&self, count: i64) -> bool;
    fn is_max_attempts(&self, count: i64) -> bool;
    async fn increment_rate_limit(&self, email: &Email) -> Result<i64, anyhow::Error>;
    async fn increment_attempts(&self, email: &Email) -> Result<i64, anyhow::Error>;
    async fn save(&self, email: &Email, data: &VerificationData) -> Result<(), anyhow::Error>;
    async fn find(&self, email: &Email) -> Result<Option<VerificationData>, anyhow::Error>;
    async fn delete(&self, email: &Email) -> Result<(), anyhow::Error>;
}
