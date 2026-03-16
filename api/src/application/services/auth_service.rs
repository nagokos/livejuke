use crate::domain::{
    models::user::NewEmailUser,
    repositories::{auth_repository::AuthRepository, user_repository::UserRepository},
};

pub struct AuthService<A, U>
where
    A: AuthRepository,
    U: UserRepository,
{
    user_repo: U,
    auth_repo: A,
}

impl<A, U> AuthService<A, U>
where
    A: AuthRepository,
    U: UserRepository,
{
    pub fn new(auth_repo: A, user_repo: U) -> Self {
        Self {
            user_repo,
            auth_repo,
        }
    }
    pub async fn register_by_email(&self, new_email_user: &NewEmailUser) {}
}
