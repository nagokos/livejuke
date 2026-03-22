use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    domain::authentication::error::AuthenticationError, presentation::error_code::ErrorCode,
};

impl IntoResponse for AuthenticationError {
    fn into_response(self) -> axum::response::Response {
        let (status, code): (StatusCode, ErrorCode) = match self {
            Self::EmailAlreadyExists => (StatusCode::CONFLICT, ErrorCode::EmailAlreadyExists),
            Self::Email(_) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidEmail),
            Self::Password(_) => (StatusCode::BAD_REQUEST, ErrorCode::InvalidPassword),
            Self::AuthenticationFailed => (StatusCode::UNAUTHORIZED, ErrorCode::Unauthorized),
        };
        (
            status,
            Json(json!({ "code": code.as_str(),"error": self.to_string() })),
        )
            .into_response()
    }
}
