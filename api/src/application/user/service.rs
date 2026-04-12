use std::sync::Arc;

use crate::{
    application::error::AppError,
    domain::{
        id::Id,
        user::{error::UserError, model::User, repository::UserRepository},
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
}
