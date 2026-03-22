pub enum ErrorCode {
    EmailAlreadyExists,
    InvalidEmail,
    InvalidPassword,
    InvalidDisplayName,
    RateLimitExceeded,
    InternalError,
    Unauthorized,
}

impl ErrorCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmailAlreadyExists => "EMAIL_ALREADY_EXISTS",
            Self::InvalidEmail => "INVALID_EMAIL",
            Self::InvalidPassword => "INVALID_PASSWORD",
            Self::InvalidDisplayName => "INVALID_DISPLAY_NAME",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::InternalError => "INTERNAL_ERROR",
            Self::Unauthorized => "UNAUTHORIZED",
        }
    }
}
