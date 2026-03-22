use crate::domain::user::display_name::DisplayNameError;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error(transparent)]
    DisplayName(#[from] DisplayNameError),
}
