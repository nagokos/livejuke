use crate::{
    application::{
        error::AppError,
        traits::{password_hasher::PasswordHasher, token_provider::TokenProvider},
    },
    domain::{
        authentication::{
            model::{EmailCredentials, NewAuthentication, Provider},
            repository::AuthRepository,
        },
        user::{
            model::{NewUser, User},
            repository::UserRepository,
        },
    },
};

pub struct AuthService<A, U, P, T>
where
    A: AuthRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    P: PasswordHasher + Send + Sync,
    T: TokenProvider + Send + Sync,
{
    user_repo: U,
    auth_repo: A,
    password_hasher: P,
    token_provider: T,
}

impl<A, U, P, T> AuthService<A, U, P, T>
where
    A: AuthRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    P: PasswordHasher + Send + Sync,
    T: TokenProvider + Send + Sync,
{
    pub fn new(auth_repo: A, user_repo: U, password_hasher: P, token_provider: T) -> Self {
        Self {
            user_repo,
            auth_repo,
            password_hasher,
            token_provider,
        }
    }
    pub async fn register_by_email(
        &self,
        new_user: NewUser,
        credentials: EmailCredentials,
    ) -> Result<(User, String), AppError> {
        let password_digest = self.password_hasher.hash(&credentials.password)?;
        let new_authentication = NewAuthentication::new(
            Provider::Email,
            credentials.email.into_inner(),
            Some(password_digest),
        );
        let user = self
            .auth_repo
            .create_user_with_authentication(new_user, new_authentication)
            .await
            .map_err(|e| match e.downcast() {
                Ok(auth_err) => AppError::Authentication(auth_err),
                Err(e) => AppError::Unexpected(e),
            })?;

        let token = self.token_provider.generate(user.id, user.role)?;
        Ok((user, token))
    }
}
