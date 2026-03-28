use crate::domain::session::model::{NewSession, Session};

pub trait SessionRepository {
    fn create(
        &self,
        new_session: NewSession,
    ) -> impl Future<Output = Result<Session, anyhow::Error>> + Send;
}
