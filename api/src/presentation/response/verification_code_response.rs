use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct VerificationCodeResponse {
    resend_cooldown_seconds: u8,
}

impl VerificationCodeResponse {
    pub fn new(resend_cooldown_seconds: u8) -> Self {
        Self {
            resend_cooldown_seconds,
        }
    }
}
