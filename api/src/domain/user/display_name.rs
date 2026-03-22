use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum DisplayNameError {
    #[error("display_name address is empty")]
    Empty,
    #[error("display_name address is long")]
    TooLong,
}

#[nutype(sanitize(trim), validate(with = validate_display_name, error = DisplayNameError), derive(Deserialize, Debug, Clone))]
pub struct DisplayName(String);

fn validate_display_name(s: &str) -> Result<(), DisplayNameError> {
    if s.is_empty() {
        return Err(DisplayNameError::Empty);
    }

    if s.chars().count() > 30 {
        return Err(DisplayNameError::TooLong);
    }

    Ok(())
}
