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
    InvalidMediaType,
    InvalidGoogleToken,
    GoogleEmailNotVerified,
    EmailAlreadyInUse,
    UserNotFound,
    GlobalRateLimited,
    SendCodeRateLimited,
    SessionCreationFailed,
    NoUpdatesProvided,
    InternalError,
    Unauthorized,
}
