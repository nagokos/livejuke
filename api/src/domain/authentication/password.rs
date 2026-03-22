use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("password is short")]
    TooShort,
    #[error("password is long")]
    TooLong,
}

#[nutype(validate(with = validate_password, error = PasswordError), derive(Deserialize, Debug, Clone))]
pub struct Password(String);

fn validate_password(s: &str) -> Result<(), PasswordError> {
    if s.len() < 8 {
        return Err(PasswordError::TooShort);
    }

    if s.len() > 128 {
        return Err(PasswordError::TooLong);
    }

    Ok(())
}
