use serde::Serialize;
use utoipa::ToSchema;

use crate::{application::auth::dto::AuthResult, domain::user::model::User};

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    user: UserResponse,
    access_token: String,
    refresh_token: String,
}

impl From<AuthResult> for AuthResponse {
    fn from(value: AuthResult) -> Self {
        Self {
            user: value.user.into(),
            access_token: value.access_token.to_string(),
            refresh_token: value.refresh_token.to_string(),
        }
    }
}

#[derive(Serialize, ToSchema)]
struct UserResponse {
    id: i64,
    email: String,
    display_name: String,
    role: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.get(),
            email: value.email,
            display_name: value.display_name,
            role: value.role.as_str().to_string(),
        }
    }
}
