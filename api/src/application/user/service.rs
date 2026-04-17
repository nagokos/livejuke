use std::sync::Arc;

use crate::{
    application::error::AppError,
    domain::{
        id::Id,
        user::{
            error::UserError,
            model::{UpdateUserPayload, User, UserAuthDetail},
            repository::UserRepository,
        },
    },
};

pub struct UserService {
    pub user_repo: Arc<dyn UserRepository>,
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
}
