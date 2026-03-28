use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    domain::session::error::SessionError,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

impl IntoResponse for SessionError {
    fn into_response(self) -> axum::response::Response {
        let (code, error) = match self {
            Self::CreationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: ErrorCode::SessionCreationFailed,
                    message: self.to_string(),
                },
            ),
        };
        (code, Json(error)).into_response()
    }
}
