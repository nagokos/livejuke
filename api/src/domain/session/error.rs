#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("session creation failed")]
    CreationFailed,
}
