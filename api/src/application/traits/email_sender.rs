use crate::domain::authentication::email::Email;
use async_trait::async_trait;

#[async_trait]
pub trait EmailSender: Send + Sync {
    async fn send(&self, to: &Email, subject: &str, body: &str) -> Result<(), anyhow::Error>;
}
