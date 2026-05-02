use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::{
        error::AppError,
        traits::{
            email_sender::EmailSender, object_store::ObjectStore, types::VerificationData,
            upload_session_store::UploadSessionStore,
            verification_code_store::VerificationCodeStore,
        },
    },
    domain::{
        authentication::{email::Email, error::AuthenticationError},
        id::Id,
        shared::media_type::MediaType,
        user::{
            error::UserError,
            model::{UpdateUserPayload, User, UserAuthDetail},
            repository::UserRepository,
        },
    },
};

pub struct UserRepositories {
    pub user_repo: Arc<dyn UserRepository>,
}

pub struct UserProviders {
    pub object_store: Arc<dyn ObjectStore>,
    pub upload_session_store: Arc<dyn UploadSessionStore>,
    pub verification_code_store: Arc<dyn VerificationCodeStore>,
    pub email_sender: Arc<dyn EmailSender>,
}

pub struct UserService {
    pub repos: UserRepositories,
    pub providers: UserProviders,
}

impl UserService {
    pub fn new(repos: UserRepositories, providers: UserProviders) -> Self {
        Self { repos, providers }
    }
    pub async fn get_user(&self, user_id: Id<User>) -> Result<UserAuthDetail, AppError> {
        let user = self
            .repos
            .user_repo
            .find_user_with_auth_status(user_id)
            .await?;
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
        let user = self.repos.user_repo.update(user_id, update_user).await?;
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
            .providers
            .object_store
            .get_presigned_uri(format!("avatars/{}", key), media_type.as_ref())
            .await?;

        self.providers
            .upload_session_store
            .set_pending_upload(user_id, &key)
            .await?;

        Ok(presigned_uri)
    }
    pub async fn update_avatar(&self, user_id: Id<User>) -> Result<User, AppError> {
        let old_key = self
            .repos
            .user_repo
            .find_by_id(user_id)
            .await?
            .ok_or(UserError::NotFound)?
            .avatar_key;

        let key = self
            .providers
            .upload_session_store
            .get_pending_upload(user_id)
            .await?;

        let payload = UpdateUserPayload::default().avatar_key(key);
        let user = self.repos.user_repo.update(user_id, payload).await?;

        if let Some(old_key) = old_key
            && let Err(e) = self
                .providers
                .object_store
                .remove_object(format!("avatars/{}", old_key))
                .await
        {
            tracing::error!(error = %e, "failed to remove s3 object {}", old_key);
        };

        Ok(user)
    }
    pub async fn send_code(&self, user_id: Id<User>) -> Result<(), AppError> {
        let Some(user) = self.repos.user_repo.find_by_id(user_id).await? else {
            return Err(AuthenticationError::AuthenticationFailed.into());
        };

        let email = unsafe { Email::new_unchecked(user.email) };
        let count = self
            .providers
            .verification_code_store
            .increment_rate_limit(&email)
            .await?;
        if self
            .providers
            .verification_code_store
            .is_rate_limited(count)
        {
            return Err(AuthenticationError::TooManyRequests.into());
        }

        let verification_data = VerificationData::new();
        self.providers
            .verification_code_store
            .save(&email, &verification_data)
            .await?;

        let body = format!(
            "LiveJukeアカウントの削除を申請しました。 確認コード: {}",
            &verification_data.code
        );

        self.providers
            .email_sender
            .send(&email, "アカウント削除の確認コード", body)
            .await?;

        Ok(())
    }
    pub async fn delete_user(&self, user_id: Id<User>, code: String) -> Result<(), AppError> {
        let Some(user) = self.repos.user_repo.find_by_id(user_id).await? else {
            return Err(AuthenticationError::AuthenticationFailed.into());
        };

        let email = unsafe { Email::new_unchecked(user.email) };

        let Some(data) = self.providers.verification_code_store.find(&email).await? else {
            return Err(AuthenticationError::InvalidVerificationCode.into());
        };

        if data.code != code {
            let count = self
                .providers
                .verification_code_store
                .increment_attempts(&email)
                .await?;
            if self
                .providers
                .verification_code_store
                .is_max_attempts(count)
            {
                self.providers
                    .verification_code_store
                    .delete(&email)
                    .await?
            }
            return Err(AuthenticationError::InvalidVerificationCode.into());
        }

        self.repos.user_repo.delete(user_id).await?;

        self.providers
            .verification_code_store
            .delete(&email)
            .await?;

        Ok(())
    }
}
