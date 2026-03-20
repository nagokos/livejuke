use utoipa::ToSchema;

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("invalid email format")]
    InvalidEmail,
    #[error("password too short")]
    PasswordTooShort,
}
