use crate::domain::{
    authentication::model::NewAuthentication,
    user::model::{NewUser, User},
};

pub trait AuthRepository {
    fn create_user_with_authentication(
        &self,
        new_user: NewUser,
        new_authentication: NewAuthentication,
    ) -> impl Future<Output = Result<User, anyhow::Error>> + Send;
}
