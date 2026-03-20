pub enum ErrorCode {
    EmailAlreadyExists,
    InvalidEmail,
    InvalidPassword,
    RateLimitExceeded,
    InternalError,
}

impl ErrorCode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::EmailAlreadyExists => "EMAIL_ALREADY_EXISTS",
            Self::InvalidEmail => "INVALID_EMAIL",
            Self::InvalidPassword => "INVALID_PASSWORD",
            Self::RateLimitExceeded => "RATE_LIMIT_EXCEEDED",
            Self::InternalError => "INTERNAL_ERROR",
        }
    }
}
