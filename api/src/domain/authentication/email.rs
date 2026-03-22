use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("Please include the '@' symbol in your email address")]
    MissingAtSign,
    #[error("email address is short")]
    TooShort,
}

#[nutype(sanitize(trim, lowercase), validate(with = validate_email, error = EmailError), derive(Deserialize, Debug, Clone))]
pub struct Email(String);

fn validate_email(s: &str) -> Result<(), EmailError> {
    if !s.contains("@") {
        return Err(EmailError::MissingAtSign);
    }

    if s.len() < 5 {
        return Err(EmailError::TooShort);
    }

    Ok(())
}
