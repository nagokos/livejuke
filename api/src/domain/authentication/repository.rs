use async_trait::async_trait;

use crate::domain::{
    authentication::model::{Authentication, AuthenticationPayload, Provider},
    id::Id,
    user::model::{User, UserPayload},
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn create_user_with_authentication(
        &self,
        user: UserPayload,
        authentication_provider: AuthenticationPayload,
    ) -> Result<User, anyhow::Error>;
    async fn update_user_with_authentication(
        &self,
        user_id: Id<User>,
        authentication_provider: AuthenticationPayload,
    ) -> Result<User, anyhow::Error>;
    async fn find_by_provider_uid(
        &self,
        provider: Provider,
        uid: &str,
    ) -> Result<Option<Authentication>, anyhow::Error>;
}
