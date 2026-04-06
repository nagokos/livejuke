use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct SendCodeInput {
    pub email: String,
}

#[derive(Deserialize, ToSchema)]
pub struct VerifyCodeInput {
    pub email: String,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct AuthGoogleInput {
    pub id_token: String,
    pub device_info: DeviceInfoInput,
}

#[derive(Deserialize, ToSchema)]
pub struct DeviceInfoInput {
    pub device_name: Option<String>,
    pub model_name: Option<String>,
    pub os: String,
}
