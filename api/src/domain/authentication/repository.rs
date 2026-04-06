use async_trait::async_trait;

use crate::domain::{
    authentication::model::{Authentication, NewAuthentication, Provider},
    user::model::{NewUser, User},
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_user_with_authentication(
        &self,
        new_user: NewUser,
        new_authentication: NewAuthentication,
    ) -> Result<User, anyhow::Error>;
    async fn find_by_provider_uid(
        &self,
        provider: Provider,
        uid: &str,
    ) -> Result<Option<Authentication>, anyhow::Error>;
}
