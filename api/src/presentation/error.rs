use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{application::error::AppError, presentation::error_code::ErrorCode};

pub mod authentication_error;
pub mod session_error;
pub mod user_error;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: ErrorCode,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::User(e) => e.into_response(),
            Self::Authentication(e) => e.into_response(),
            Self::Session(e) => e.into_response(),
            Self::Unexpected(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    code: ErrorCode::InternalError,
                    message: "internal server error".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
