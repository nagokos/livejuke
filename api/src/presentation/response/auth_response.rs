use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    application::auth::dto::AuthResult,
    domain::{authentication::model::Provider, user::model::UserAuthDetail},
    presentation::response::user_response::CurrentUserResponse,
};

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    access_token: String,
    refresh_token: String,
}

impl From<AuthResult> for AuthResponse {
    fn from(value: AuthResult) -> Self {
        Self {
            access_token: value.access_token.to_string(),
            refresh_token: value.refresh_token.to_string(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserAuthDetailResponse {
    user: CurrentUserResponse,
    auth_status: AuthStatusResponse,
}

impl UserAuthDetailResponse {
    pub fn from_domain(value: UserAuthDetail, cdn_base_url: String) -> Self {
        Self {
            user: CurrentUserResponse::from_domain(value.user, cdn_base_url),
            auth_status: value.linked_providers.into(),
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct AuthStatusResponse {
    is_google_linked: bool,
    is_email_linked: bool,
}

impl From<Vec<Provider>> for AuthStatusResponse {
    fn from(value: Vec<Provider>) -> Self {
        Self {
            is_google_linked: value.contains(&Provider::Google),
            is_email_linked: value.contains(&Provider::Email),
        }
    }
}
