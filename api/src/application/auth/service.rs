use chrono::{Duration, Utc};

use crate::{
    application::{
        auth::{config::AuthConfig, dto::AuthResult},
        error::AppError,
        traits::{
            access_token_provider::AccessTokenProvider,
            id_token_verifier::IdTokenVerifier,
            password_hasher::PasswordHasher,
            refresh_token_provider::RefreshTokenProvider,
            types::{AccessToken, RefreshToken},
        },
    },
    domain::{
        authentication::{
            error::AuthenticationError,
            model::{EmailCredentials, NewAuthentication, Provider},
            repository::AuthRepository,
        },
        session::{
            error::SessionError,
            model::{DeviceInfo, NewSession},
            repository::SessionRepository,
        },
        user::{
            model::{NewUser, User},
            repository::UserRepository,
        },
    },
};

pub struct AuthProviders<P, T, R, I> {
    pub password_hasher: P,
    pub access_token_provider: T,
    pub refresh_token_provider: R,
    pub id_token_verifier: I,
}

pub struct AuthRepositories<A, U, S> {
    pub auth_repo: A,
    pub user_repo: U,
    pub session_repo: S,
}

pub struct AuthService<A, U, S, P, T, R, I>
where
    A: AuthRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
    P: PasswordHasher + Send + Sync,
    T: AccessTokenProvider + Send + Sync,
    R: RefreshTokenProvider + Send + Sync,
    I: IdTokenVerifier + Send + Sync,
{
    repos: AuthRepositories<A, U, S>,
    providers: AuthProviders<P, T, R, I>,
    config: AuthConfig,
}

impl<A, U, S, P, T, R, I> AuthService<A, U, S, P, T, R, I>
where
    A: AuthRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    S: SessionRepository + Send + Sync,
    P: PasswordHasher + Send + Sync,
    T: AccessTokenProvider + Send + Sync,
    R: RefreshTokenProvider + Send + Sync,
    I: IdTokenVerifier + Send + Sync,
{
    pub fn new(
        repos: AuthRepositories<A, U, S>,
        providers: AuthProviders<P, T, R, I>,
        config: AuthConfig,
    ) -> Self {
        Self {
            repos,
            providers,
            config,
        }
    }
    pub async fn register_by_email(
        &self,
        new_user: NewUser,
        credentials: EmailCredentials,
        device_info: DeviceInfo,
    ) -> Result<AuthResult, AppError> {
        let password_digest = self
            .providers
            .password_hasher
            .hash(&credentials.password.into_inner())?;
        let new_authentication = NewAuthentication::new(
            Provider::Email,
            credentials.email.into_inner(),
            Some(password_digest),
        );
        let user = self
            .repos
            .auth_repo
            .create_user_with_authentication(new_user, new_authentication)
            .await
            .map_err(|e| match e.downcast() {
                Ok(auth_err) => AppError::Authentication(auth_err),
                Err(e) => AppError::Unexpected(e),
            })?;

        let (access_token, refresh_token) = self.create_session(&user, device_info).await?;

        Ok(AuthResult {
            user,
            access_token,
            refresh_token,
        })
    }
    pub async fn login_by_email(
        &self,
        credentials: EmailCredentials,
        device_info: DeviceInfo,
    ) -> Result<AuthResult, AppError> {
        let authentication = self
            .repos
            .auth_repo
            .find_by_provider_uid(Provider::Email, &credentials.email.into_inner())
            .await?
            .ok_or(AuthenticationError::AuthenticationFailed)?;
        if !self.providers.password_hasher.verify(
            &credentials.password.into_inner(),
            &authentication
                .password_digest
                .ok_or(AuthenticationError::AuthenticationFailed)?,
        )? {
            return Err(AuthenticationError::AuthenticationFailed.into());
        }
        let user = self
            .repos
            .user_repo
            .find_by_id(authentication.user_id)
            .await?
            .ok_or(AuthenticationError::AuthenticationFailed)?;

        let (access_token, refresh_token) = self.create_session(&user, device_info).await?;

        Ok(AuthResult {
            user,
            access_token,
            refresh_token,
        })
    }
    pub async fn auth_google(
        &self,
        id_token: String,
        device_info: DeviceInfo,
    ) -> Result<AuthResult, AppError> {
        let user_info = self.providers.id_token_verifier.verify(&id_token).await?;
        let authentication = self
            .repos
            .auth_repo
            .find_by_provider_uid(Provider::Google, &user_info.sub)
            .await?;

        let user = if let Some(authentication) = authentication {
            self.repos
                .user_repo
                .find_by_id(authentication.user_id)
                .await?
                .ok_or(AuthenticationError::AuthenticationFailed)?
        } else {
            let new_user = {
                let display_name = if user_info.name.len() > 30 {
                    user_info.name.chars().take(30).collect()
                } else {
                    user_info.name
                };
                NewUser::try_new(display_name, user_info.email)?
            };
            let new_authentication = NewAuthentication::new(Provider::Google, user_info.sub, None);
            self.repos
                .auth_repo
                .create_user_with_authentication(new_user, new_authentication)
                .await
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to create_user_with_authentication");
                    match e.downcast() {
                        Ok(auth_err) => AppError::Authentication(auth_err),
                        Err(e) => AppError::Unexpected(e),
                    }
                })?
        };

        let (access_token, refresh_token) = self.create_session(&user, device_info).await?;

        Ok(AuthResult {
            user,
            access_token,
            refresh_token,
        })
    }
    async fn create_session(
        &self,
        user: &User,
        device_info: DeviceInfo,
    ) -> Result<(AccessToken, RefreshToken), SessionError> {
        let access_token = self
            .providers
            .access_token_provider
            .generate(user.id, user.role)
            .map_err(|e| {
                tracing::error!(error = %e, "failed to session creation");
                SessionError::CreationFailed
            })?;

        let refresh_token = self.providers.refresh_token_provider.generate();
        let hashed = self.providers.refresh_token_provider.hash(&refresh_token);

        let new_session = NewSession::new(
            user.id,
            hashed,
            device_info,
            Utc::now() + Duration::seconds(self.config.refresh_token_expiration),
        );

        self.repos
            .session_repo
            .create(new_session)
            .await
            .map_err(|e| {
                tracing::error!(error = %e, "failed to session creation");
                SessionError::CreationFailed
            })?;

        Ok((access_token, refresh_token))
    }
}

// #[cfg(test)]
// mod tests {
//     use chrono::Utc;
//
//     use crate::domain::{
//         authentication::{email::Email, model::Authentication, password::Password},
//         id::Id,
//         user::{display_name::DisplayName, model::Role},
//     };
//
//     use super::*;
//
//     struct MockAuthRepository {
//         user: User,
//         authentication: Authentication,
//         should_fail: bool,
//         is_none: bool,
//     }
//     impl AuthRepository for MockAuthRepository {
//         async fn create_user_with_authentication(
//             &self,
//             _new_user: NewUser,
//             _new_authentication: NewAuthentication,
//         ) -> Result<User, anyhow::Error> {
//             if self.should_fail {
//                 return Err(AuthenticationError::EmailAlreadyExists.into());
//             }
//             Ok(self.user.clone())
//         }
//         async fn find_by_provider_uid(
//             &self,
//             _provider: Provider,
//             _uid: &str,
//         ) -> Result<Option<Authentication>, anyhow::Error> {
//             if self.is_none {
//                 return Ok(None);
//             }
//             Ok(Some(self.authentication.clone()))
//         }
//     }
//
//     struct MockPasswordHasher {
//         verify_result: bool,
//     }
//     impl PasswordHasher for MockPasswordHasher {
//         fn hash(&self, _password: &str) -> Result<String, anyhow::Error> {
//             Ok("hashed_password".to_string())
//         }
//         fn verify(&self, _password: &str, _password_hash: &str) -> Result<bool, anyhow::Error> {
//             Ok(self.verify_result)
//         }
//     }
//
//     struct MockTokenProvider;
//     impl AccessTokenProvider for MockTokenProvider {
//         fn generate(
//             &self,
//             _sub: crate::domain::id::Id<User>,
//             _role: crate::domain::user::model::Role,
//         ) -> Result<String, anyhow::Error> {
//             Ok("mock_jwt_token".to_string())
//         }
//     }
//
//     struct MockUserRepository {
//         user: User,
//         is_none: bool,
//     }
//     impl UserRepository for MockUserRepository {
//         async fn find_by_id(&self, _user_id: Id<User>) -> Result<Option<User>, anyhow::Error> {
//             if self.is_none {
//                 return Ok(None);
//             }
//             Ok(Some(self.user.clone()))
//         }
//     }
//
//     fn test_user() -> User {
//         User {
//             id: Id::new(1),
//             display_name: "test".to_string(),
//             role: Role::User,
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }
//
//     fn test_authentication_provider_email() -> Authentication {
//         Authentication {
//             id: Id::new(1),
//             user_id: Id::new(1),
//             provider: Provider::Email,
//             uid: "test@example.com".to_string(),
//             password_digest: "".to_string(),
//             created_at: Utc::now(),
//             updated_at: Utc::now(),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_register_by_email_success() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: false,
//                 user: test_user(),
//                 authentication: test_authentication_provider_email(),
//                 is_none: false,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockPasswordHasher {
//                 verify_result: true,
//             },
//             MockTokenProvider,
//         );
//
//         let new_user = NewUser {
//             display_name: DisplayName::try_new("test".to_string()).unwrap(),
//         };
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.register_by_email(new_user, credentials).await;
//
//         assert!(result.is_ok());
//         let (user, token) = result.unwrap();
//         assert_eq!(user.display_name.as_str(), "test");
//         assert_eq!(token.as_str(), "mock_jwt_token");
//     }
//
//     #[tokio::test]
//     async fn test_register_by_email_returns_email_already_exists() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: true,
//                 user: test_user(),
//                 authentication: test_authentication_provider_email(),
//                 is_none: false,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockPasswordHasher {
//                 verify_result: true,
//             },
//             MockTokenProvider,
//         );
//
//         let new_user = NewUser {
//             display_name: DisplayName::try_new("test".to_string()).unwrap(),
//         };
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.register_by_email(new_user, credentials).await;
//
//         assert!(result.is_err());
//         assert!(matches!(
//             result.unwrap_err(),
//             AppError::Authentication(AuthenticationError::EmailAlreadyExists)
//         ))
//     }
//
//     #[tokio::test]
//     async fn test_login_by_email_success() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: false,
//                 authentication: test_authentication_provider_email(),
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockPasswordHasher {
//                 verify_result: true,
//             },
//             MockTokenProvider,
//         );
//
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.login_by_email(credentials).await;
//
//         assert!(result.is_ok());
//         let (user, token) = result.unwrap();
//         assert_eq!(user.display_name.as_str(), "test");
//         assert_eq!(token.as_str(), "mock_jwt_token");
//     }
//
//     #[tokio::test]
//     async fn test_login_by_email_password_mismatch() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: false,
//                 authentication: test_authentication_provider_email(),
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockPasswordHasher {
//                 verify_result: false,
//             },
//             MockTokenProvider,
//         );
//
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.login_by_email(credentials).await;
//
//         assert!(result.is_err());
//         assert!(matches!(
//             result.unwrap_err(),
//             AppError::Authentication(AuthenticationError::AuthenticationFailed)
//         ))
//     }
//
//     #[tokio::test]
//     async fn test_login_by_email_return_user_not_found() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: false,
//                 authentication: test_authentication_provider_email(),
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: true,
//             },
//             MockPasswordHasher {
//                 verify_result: true,
//             },
//             MockTokenProvider,
//         );
//
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.login_by_email(credentials).await;
//
//         assert!(result.is_err());
//         assert!(matches!(
//             result.unwrap_err(),
//             AppError::Authentication(AuthenticationError::AuthenticationFailed)
//         ))
//     }
//
//     #[tokio::test]
//     async fn test_login_by_email_authentication_not_found() {
//         let service = AuthService::new(
//             MockAuthRepository {
//                 should_fail: false,
//                 authentication: test_authentication_provider_email(),
//                 user: test_user(),
//                 is_none: true,
//             },
//             MockUserRepository {
//                 user: test_user(),
//                 is_none: false,
//             },
//             MockPasswordHasher {
//                 verify_result: true,
//             },
//             MockTokenProvider,
//         );
//
//         let credentials = EmailCredentials {
//             email: Email::try_new("test@example.com".to_string()).unwrap(),
//             password: Password::try_new("password0123".to_string()).unwrap(),
//         };
//
//         let result = service.login_by_email(credentials).await;
//
//         assert!(result.is_err());
//         assert!(matches!(
//             result.unwrap_err(),
//             AppError::Authentication(AuthenticationError::AuthenticationFailed)
//         ))
//     }
// }
