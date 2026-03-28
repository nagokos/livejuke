use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    EmailAlreadyExists,
    InvalidEmail,
    InvalidPassword,
    InvalidDisplayName,
    RateLimitExceeded,
    InternalError,
    Unauthorized,
}
