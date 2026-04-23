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
    #[error("email authentication required")]
    EmailAuthenticationRequired,
    #[error("google account already in use")]
    GoogleAccountAlreadyInUse,
    #[error("too many requests")]
    TooManyRequests,
}
