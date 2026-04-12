use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::user::model::User;

#[derive(Serialize, ToSchema)]
pub struct CurrentUserResponse {
    id: i64,
    email: String,
    display_name: String,
    role: String,
}

impl From<User> for CurrentUserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.get(),
            email: value.email,
            display_name: value.display_name,
            role: value.role.as_str().to_string(),
        }
    }
}
