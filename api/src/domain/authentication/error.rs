use crate::domain::authentication::email::EmailError;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Email(#[from] EmailError),
    #[error("invalid verification code")]
    InvalidVerificationCode,
    #[error("invalid verification code")]
    InvalidRefreshToken,
    #[error("authentication failed")]
    AuthenticationFailed,
    #[error("too many requests")]
    TooManyRequests,
}
