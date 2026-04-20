use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::{
        error::AppError,
        traits::{object_store::ObjectStore, upload_session_store::UploadSessionStore},
    },
    domain::{
        id::Id,
        shared::media_type::MediaType,
        user::{
            error::UserError,
            model::{UpdateUserPayload, User, UserAuthDetail},
            repository::UserRepository,
        },
    },
};

pub struct UserService {
    pub user_repo: Arc<dyn UserRepository>,
    pub object_store: Arc<dyn ObjectStore>,
    pub upload_session_store: Arc<dyn UploadSessionStore>,
}

impl UserService {
    pub async fn get_user(&self, user_id: Id<User>) -> Result<UserAuthDetail, AppError> {
        let user = self.user_repo.find_user_with_auth_status(user_id).await?;
        Ok(user)
    }
    pub async fn update_user(
        &self,
        user_id: Id<User>,
        update_user: UpdateUserPayload,
    ) -> Result<User, AppError> {
        if update_user.is_empty() {
            return Err(UserError::EmptyUpdate.into());
        }
        let user = self.user_repo.update(user_id, update_user).await?;
        Ok(user)
    }
    pub async fn presigned_uri(
        &self,
        user_id: Id<User>,
        media_type: MediaType,
    ) -> Result<String, AppError> {
        let uuid = Uuid::new_v4();
        let key = format!("{}.{}", uuid, media_type.extention());

        let presigned_uri = self
            .object_store
            .get_presigned_uri(&key, media_type.as_ref())
            .await?;

        self.upload_session_store
            .set_pending_upload(user_id, &key)
            .await?;

        Ok(presigned_uri)
    }
    pub async fn update_avatar(&self, user_id: Id<User>) -> Result<User, AppError> {
        let avatar_key = self
            .upload_session_store
            .get_pending_upload(user_id)
            .await?;

        let payload = UpdateUserPayload::default().avatar_key(avatar_key);
        let user = self.user_repo.update(user_id, payload).await?;

        Ok(user)
    }
}
