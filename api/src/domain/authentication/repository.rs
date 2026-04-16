use async_trait::async_trait;

use crate::domain::{
    authentication::model::{Authentication, AuthenticationProvider, Provider},
    user::model::{UpdateUserProvider, User, UserProvider},
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_user_with_authentication(
        &self,
        new_user: UserProvider,
        new_authentication: AuthenticationProvider,
    ) -> Result<User, anyhow::Error>;
    async fn update_user_with_authentication(
        &self,
        update_user: UpdateUserProvider,
        new_authentication: AuthenticationProvider,
    ) -> Result<User, anyhow::Error>;
    async fn find_by_provider_uid(
        &self,
        provider: Provider,
        uid: &str,
    ) -> Result<Option<Authentication>, anyhow::Error>;
}
