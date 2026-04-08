#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("session creation failed")]
    CreationFailed,
}
