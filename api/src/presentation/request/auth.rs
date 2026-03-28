use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct RegisterEmailInput {
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub device_info: DeviceInfoInput,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginEmailInput {
    pub email: String,
    pub password: String,
    pub device_info: DeviceInfoInput,
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
