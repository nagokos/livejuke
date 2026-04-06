use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    domain::authentication::error::AuthenticationError,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let (code, error) = match self {
            Self::Email(_) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::InvalidEmail,
                    message: self.to_string(),
                },
            ),
            Self::InvalidVerificationCode => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: ErrorCode::InvalidVerificationCode,
                    message: self.to_string(),
                },
            ),
            Self::TooManyRequests => (
                StatusCode::TOO_MANY_REQUESTS,
                ErrorResponse {
                    code: ErrorCode::RateLimitExceeded,
                    message: self.to_string(),
                },
            ),
            Self::AuthenticationFailed => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: ErrorCode::Unauthorized,
                    message: self.to_string(),
                },
            ),
        };
        (code, Json(error)).into_response()
    }
}
