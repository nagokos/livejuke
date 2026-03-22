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
        let password_digest = self
            .password_hasher
            .hash(&credentials.password.into_inner())?;
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
#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::domain::{authentication::email::Email, id::Id, user::model::Role};

    use super::*;

    struct MockAuthRepository {
        user: User,
        should_fail: bool,
    }
    impl AuthRepository for MockAuthRepository {
        async fn create_user_with_authentication(
            &self,
            _new_user: NewUser,
            _new_authentication: NewAuthentication,
        ) -> Result<User, anyhow::Error> {
            if self.should_fail {
                Err(AuthenticationError::EmailAlreadyExists.into())
            } else {
                Ok(self.user.clone())
            }
        }
        async fn find_by_provider_uid(
            &self,
            _provider: Provider,
            _uid: &str,
        ) -> Result<Option<crate::domain::authentication::model::Authentication>, anyhow::Error>
        {
            unimplemented!()
        }
    }

    struct MockPasswordHasher;
    impl PasswordHasher for MockPasswordHasher {
        fn hash(&self, _password: &str) -> Result<String, anyhow::Error> {
            Ok("hashed_password".to_string())
        }
        fn verify(&self, _password: &str, _password_hash: &str) -> Result<bool, anyhow::Error> {
            unimplemented!()
        }
    }

    struct MockTokenProvider;
    impl TokenProvider for MockTokenProvider {
        fn generate(
            &self,
            _sub: crate::domain::id::Id<User>,
            _role: crate::domain::user::model::Role,
        ) -> Result<String, anyhow::Error> {
            Ok("mock_jwt_token".to_string())
        }
    }

    struct MockUserRepository;
    impl UserRepository for MockUserRepository {
        async fn find_by_id(&self, _user_id: Id<User>) -> Result<Option<User>, anyhow::Error> {
            unimplemented!()
        }
    }

    fn test_user() -> User {
        User {
            id: Id::new(1),
            display_name: "test".to_string(),
            role: Role::User,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_register_by_email_success() {
        let service = AuthService::new(
            MockAuthRepository {
                should_fail: false,
                user: test_user(),
            },
            MockUserRepository,
            MockPasswordHasher,
            MockTokenProvider,
        );

        let new_user = NewUser {
            display_name: "test".to_string(),
        };
        let credentials = EmailCredentials {
            email: Email::try_new("test@example.com".to_string()).unwrap(),
            password: "password0123".to_string(),
        };

        let result = service.register_by_email(new_user, credentials).await;

        assert!(result.is_ok());
        let (user, token) = result.unwrap();
        assert_eq!(user.display_name.as_str(), "test");
        assert_eq!(token.as_str(), "mock_jwt_token");
    }
    #[tokio::test]
    async fn test_register_by_email_returns_email_already_exists() {
        let service = AuthService::new(
            MockAuthRepository {
                should_fail: true,
                user: test_user(),
            },
            MockUserRepository,
            MockPasswordHasher,
            MockTokenProvider,
        );

        let new_user = NewUser {
            display_name: "test".to_string(),
        };
        let credentials = EmailCredentials {
            email: Email::try_new("test@example.com".to_string()).unwrap(),
            password: "password0123".to_string(),
        };

        let result = service.register_by_email(new_user, credentials).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AppError::Authentication(AuthenticationError::EmailAlreadyExists)
        ))
    }
}
