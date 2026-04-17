use nutype::nutype;

#[derive(Debug, thiserror::Error)]
pub enum MediaTypeError {
    #[error("media_type is unsupported error")]
    UnsupportedMediaType,
}

#[nutype(sanitize(trim, lowercase), validate(with = validate_media_type, error = MediaTypeError), derive(Deserialize, Debug, Clone, AsRef))]
pub struct MediaType(String);

impl MediaType {
    pub fn extention(&self) -> &str {
        self.as_ref().split("/").last().unwrap()
    }
}

fn validate_media_type(s: &str) -> Result<(), MediaTypeError> {
    if s != "image/png" && s != "image/jpeg" && s != "image/webp" {
        return Err(MediaTypeError::UnsupportedMediaType);
    }

    Ok(())
}
