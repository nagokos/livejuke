use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::domain::user::error::UserError;

// impl IntoResponse for UserError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, message): (StatusCode, String) = match self {
//             UserError::InvalidEmail => (StatusCode::BAD_REQUEST, self.to_string()),
//             UserError::PasswordTooShort => (StatusCode::BAD_REQUEST, self.to_string()),
//         };
//         (status, Json(json!({ "error": message }))).into_response()
//     }
// }
