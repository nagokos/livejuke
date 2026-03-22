use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{domain::user::error::UserError, presentation::error_code::ErrorCode};

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, code) = match self {
            Self::DisplayName(_) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidDisplayName),
        };
        (
            status,
            Json(json!({ "code": code.as_str(), "error": self.to_string() })),
        )
            .into_response()
    }
}
