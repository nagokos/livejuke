use std::sync::Arc;

use crate::{
    application::error::AppError,
    domain::{
        id::Id,
        user::{
            error::UserError,
            model::{UpdateUserProvider, User},
            repository::UserRepository,
        },
    },
};

pub struct UserService {
    pub user_repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub async fn get_user(&self, user_id: Id<User>) -> Result<User, AppError> {
        let Some(user) = self.user_repo.find_by_id(user_id).await? else {
            return Err(UserError::NotFound.into());
        };
        Ok(user)
    }
    pub async fn update_user(
        &self,
        user_id: Id<User>,
        update_user: UpdateUserProvider,
    ) -> Result<User, AppError> {
        if update_user.is_empty() {
            return Err(UserError::EmptyUpdate.into());
        }
        let user = self.user_repo.update(user_id, update_user).await?;
        Ok(user)
    }
}
