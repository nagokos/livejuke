use crate::domain::authentication::email::EmailError;

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Email(#[from] EmailError),
    #[error("invalid verification code")]
    InvalidVerificationCode,
    #[error("authentication failed")]
    AuthenticationFailed,
    #[error("email already in use")]
    EmailAlreadyInUse,
    #[error("too many requests")]
    TooManyRequests,
}
