use crate::domain::{authentication::error::AuthenticationError, user::error::UserError};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // #[error(transparent)]
    // User(#[from] UserError),
    #[error(transparent)]
    Authentication(#[from] AuthenticationError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
