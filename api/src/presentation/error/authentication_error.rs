use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    domain::authentication::error::AuthenticationError, presentation::error_code::ErrorCode,
};

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let (status, code): (StatusCode, ErrorCode) = match self {
            AuthenticationError::EmailAlreadyExists => {
                (StatusCode::CONFLICT, ErrorCode::EmailAlreadyExists)
            }
            AuthenticationError::Email(_) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidEmail),
            AuthenticationError::Password(_) => {
                (StatusCode::BAD_REQUEST, ErrorCode::InvalidPassword)
            }
        };
        (
            status,
            Json(json!({ "code": code.as_str(),"error": self.to_string() })),
        )
            .into_response()
    }
}
