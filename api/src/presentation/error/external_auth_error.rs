use axum::{Json, response::IntoResponse};
use reqwest::StatusCode;

use crate::{
    application::traits::id_token_verifier::OidcVerifyError,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

impl IntoResponse for OidcVerifyError {
    fn into_response(self) -> axum::response::Response {
        let (code, error) = match self {
            Self::InvalidToken(_) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: ErrorCode::InvalidGoogleToken,
                    message: self.to_string(),
                },
            ),
            Self::EmailNotVerified => (StatusCode::UNAUTHORIZED, {
                ErrorResponse {
                    code: ErrorCode::GoogleEmailNotVerified,
                    message: self.to_string(),
                }
            }),
            Self::Network(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::InternalError,
                    message: self.to_string(),
                },
            ),
            Self::Parse(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::InternalError,
                    message: self.to_string(),
                },
            ),
            Self::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::InternalError,
                    message: self.to_string(),
                },
            ),
        };

        (code, Json(error)).into_response()
    }
}
