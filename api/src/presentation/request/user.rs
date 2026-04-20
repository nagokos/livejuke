use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UserUpdateInput {
    pub display_name: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UserAvatarUpdateInput {
    pub media_type: String,
}
