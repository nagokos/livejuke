use crate::{application::error::AppError, domain::user::display_name::DisplayNameError};

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    DisplayName(#[from] DisplayNameError),
    #[error("user not found")]
    NotFound,
}
