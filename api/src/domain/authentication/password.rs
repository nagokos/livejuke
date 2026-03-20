use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum PasswordError {
    #[error("password is short")]
    TooShort,
}

#[nutype(sanitize(lowercase), validate(with = validate_email, error = PasswordError), derive(Deserialize, Debug, Clone))]
pub struct Email(String);

fn validate_email(s: &str) -> Result<(), PasswordError> {
    if s.len() < 8 {
        return Err(PasswordError::TooShort);
    }

    Ok(())
}
