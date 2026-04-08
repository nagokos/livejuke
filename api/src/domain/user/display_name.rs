use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum DisplayNameError {
    #[error("display_name is empty")]
    Empty,
    #[error("display_name is long")]
    TooLong,
}

#[nutype(sanitize(trim), validate(with = validate_display_name, error = DisplayNameError), derive(Deserialize, Debug, Clone, AsRef))]
pub struct DisplayName(String);

fn validate_display_name(s: &str) -> Result<(), DisplayNameError> {
    if s.is_empty() {
        return Err(DisplayNameError::Empty);
    }

    if s.chars().count() > 20 {
        return Err(DisplayNameError::TooLong);
    }

    Ok(())
}
