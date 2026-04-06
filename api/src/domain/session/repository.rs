use async_trait::async_trait;

use crate::domain::session::model::{NewSession, Session};

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, new_session: NewSession) -> Result<Session, anyhow::Error>;
}
