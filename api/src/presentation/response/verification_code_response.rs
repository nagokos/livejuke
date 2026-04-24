use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SendCodeResponse {
    resend_cooldown_seconds: u8,
}

impl SendCodeResponse {
    pub fn new(resend_cooldown_seconds: u8) -> Self {
        Self {
            resend_cooldown_seconds,
        }
    }
}
