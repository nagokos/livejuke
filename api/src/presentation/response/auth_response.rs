use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    application::auth::dto::AuthResult,
    domain::{authentication::model::Provider, user::model::UserAuthDetail},
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
    id: i64,
    display_name: String,
    email: String,
    avatar_url: Option<String>,
    role: String,
    auth_status: AuthStatusResponse,
}

#[derive(Serialize, ToSchema)]
pub struct AuthStatusResponse {
    is_google_linked: bool,
    is_email_linked: bool,
}

impl UserAuthDetailResponse {
    pub fn from_domain(value: UserAuthDetail, cdn_base_url: String) -> Self {
        Self {
            id: value.user.id.get(),
            display_name: value.user.display_name,
            email: value.user.email,
            role: value.user.role.as_str().to_string(),
            avatar_url: value
                .user
                .avatar_key
                .map(|avatar_key| format!("{}/avatars/{}", cdn_base_url, avatar_key)),
            auth_status: AuthStatusResponse {
                is_google_linked: value.linked_providers.contains(&Provider::Google),
                is_email_linked: value.linked_providers.contains(&Provider::Email),
            },
        }
    }
}
