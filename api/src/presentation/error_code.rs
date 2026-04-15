use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorCode {
    InvalidEmail,
    InvalidDisplayName,
    InvalidVerificationCode,
    InvalidAccessToken,
    InvalidRefreshToken,
    UserNotFound,
    RateLimitExceeded,
    SessionCreationFailed,
    NoUpdatesProvided,
    InternalError,
    Unauthorized,
}
