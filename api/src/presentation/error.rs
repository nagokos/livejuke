use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{application::error::AppError, presentation::error_code::ErrorCode};

pub mod authentication_error;
pub mod user_error;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            // AppError::User(e) => e.into_response(),
            Self::Authentication(e) => e.into_response(),
            Self::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    json!({ "code": ErrorCode::InternalError.as_str(),"error": "internal server error" }),
                ),
            )
                .into_response(),
        }
    }
}
