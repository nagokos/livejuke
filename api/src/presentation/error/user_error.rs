use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::{
    domain::user::error::UserError,
    presentation::{error::ErrorResponse, error_code::ErrorCode},
};

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (code, error) = match self {
            Self::DisplayName(_) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: ErrorCode::InvalidDisplayName,
                    message: self.to_string(),
                },
            ),
        };
        (code, Json(error)).into_response()
    }
}
