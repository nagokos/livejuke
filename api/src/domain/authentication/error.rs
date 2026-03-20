use crate::domain::authentication::{email::EmailError, password::PasswordError};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    Email(#[from] EmailError),
    #[error(transparent)]
    Password(#[from] PasswordError),
    #[error("email already exists")]
    EmailAlreadyExists,
}
