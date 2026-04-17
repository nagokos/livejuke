use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::user::model::User;

#[derive(Serialize, ToSchema)]
pub struct CurrentUserResponse {
    id: i64,
    display_name: String,
    avatar_url: Option<String>,
    email: String,
    role: String,
}

impl CurrentUserResponse {
    pub fn from_domain(user: User, cdn_base_url: String) -> Self {
        Self {
            id: user.id.get(),
            display_name: user.display_name,
            email: user.email,
            avatar_url: user
                .avatar_key
                .map(|avatar_key| format!("{}/avatars/{}", cdn_base_url, avatar_key)),
            role: user.role.as_str().to_string(),
        }
    }
}
