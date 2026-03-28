use crate::domain::{
    authentication::error::AuthenticationError, session::error::SessionError,
    user::error::UserError,
};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    User(#[from] UserError),
    #[error(transparent)]
    Authentication(#[from] AuthenticationError),
    #[error(transparent)]
    Session(#[from] SessionError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
