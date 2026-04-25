use crate::{
    application::traits::id_token_verifier::OidcVerifyError,
    domain::{
        authentication::error::AuthenticationError, session::error::SessionError,
        shared::media_type::MediaTypeError, user::error::UserError,
    },
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
    GoogleError(#[from] OidcVerifyError),
    #[error(transparent)]
    MediaType(#[from] MediaTypeError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}
