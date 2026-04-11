use std::sync::LazyLock;

use nutype::nutype;
use regex::Regex;

static EMAIL_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-z0-9_+-]+(?:\.[a-z0-9_+-]+)*@[a-z0-9]+(?:[.-][a-z0-9]+)*\.[a-z]{2,}$").unwrap()
});

#[derive(Debug, thiserror::Error)]
pub enum EmailError {
    #[error("email address is invalid format")]
    InvalidFormat,
}

#[nutype(sanitize(trim, lowercase), validate(with = validate_email, error = EmailError), derive(Deserialize, Debug, Clone, AsRef))]
pub struct Email(String);

fn validate_email(s: &str) -> Result<(), EmailError> {
    if !EMAIL_REGEX.is_match(s) {
        return Err(EmailError::InvalidFormat);
    }

    Ok(())
}
