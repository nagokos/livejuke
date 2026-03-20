use serde::Serialize;
use utoipa::ToSchema;

use crate::domain::user::model::User;

#[derive(Serialize, ToSchema)]
pub struct CurrentUserResponse {
    pub id: i64,
    pub display_name: String,
    pub role: String,
}

impl From<User> for CurrentUserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id.get(),
            display_name: value.display_name,
            role: value.role.to_string(),
        }
    }
}
