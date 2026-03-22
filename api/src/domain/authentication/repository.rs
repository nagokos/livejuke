use crate::domain::{
    authentication::model::{Authentication, NewAuthentication, Provider},
    user::model::{NewUser, User},
};

pub trait AuthRepository {
    fn create_user_with_authentication(
        &self,
        new_user: NewUser,
        new_authentication: NewAuthentication,
    ) -> impl Future<Output = Result<User, anyhow::Error>> + Send;
    fn find_by_provider_uid(
        &self,
        provider: Provider,
        uid: &str,
    ) -> impl Future<Output = Result<Option<Authentication>, anyhow::Error>> + Send;
}
