use serde::Deserialize;
use utoipa::ToSchema;

use crate::domain::session::model::DeviceInfo;

#[derive(Deserialize, ToSchema)]
pub struct SendCodeInput {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyCodeInput {
    pub email: String,
    pub code: String,
    pub device_info: DeviceInfoInput,
}

#[derive(Deserialize, ToSchema)]
pub struct AuthGoogleInput {
    pub id_token: String,
    pub device_info: DeviceInfoInput,
}

#[derive(Deserialize, ToSchema)]
pub struct AuthRefreshInput {
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct DeviceInfoInput {
    pub device_name: Option<String>,
    pub model_name: Option<String>,
    pub os: String,
}

impl From<DeviceInfoInput> for DeviceInfo {
    fn from(value: DeviceInfoInput) -> Self {
        Self {
            device_name: value.device_name,
            model_name: value.model_name,
            os: value.os,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateEmailInput {
    pub code: String,
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LogoutInput {
    pub refresh_token: String,
}
