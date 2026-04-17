use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PresignedUriResponse {
    pub presigned_uri: String,
}
