use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    domain::authentication::error::AuthenticationError,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let (code, error) = match self {
            Self::EmailAlreadyExists => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: ErrorCode::EmailAlreadyExists,
                    message: self.to_string(),
                },
            ),
            Self::Email(_) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::InvalidEmail,
                    message: self.to_string(),
                },
            ),
            Self::Password(_) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::InvalidPassword,
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
