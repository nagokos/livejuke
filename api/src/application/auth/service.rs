use std::sync::Arc;

use chrono::{Duration, Utc};

use crate::{
    application::{
        auth::{config::AuthConfig, dto::AuthResult},
        error::AppError,
        traits::{
            access_token_provider::AccessTokenProvider,
            email_sender::EmailSender,
            id_token_verifier::IdTokenVerifier,
            refresh_token_provider::RefreshTokenProvider,
            types::{AccessToken, RefreshToken, VerificationData},
            verification_code_store::VerificationCodeStore,
        },
    },
    domain::{
        authentication::{
            email::Email,
            error::AuthenticationError,
            model::{NewAuthentication, Provider},
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

pub struct AuthRepositories {
    pub auth_repo: Arc<dyn AuthRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub session_repo: Arc<dyn SessionRepository>,
}

pub struct AuthProviders {
    pub access_token_provider: Arc<dyn AccessTokenProvider>,
    pub refresh_token_provider: Arc<dyn RefreshTokenProvider>,
    pub id_token_verifier: Arc<dyn IdTokenVerifier>,
    pub verification_code_store: Arc<dyn VerificationCodeStore>,
    pub email_sender: Arc<dyn EmailSender>,
}

pub struct AuthService {
    repos: AuthRepositories,
    providers: AuthProviders,
    config: AuthConfig,
}

impl AuthService {
    pub fn new(repos: AuthRepositories, providers: AuthProviders, config: AuthConfig) -> Self {
        Self {
            repos,
            providers,
            config,
        }
    }
    pub async fn send_verification_code(&self, email: Email) -> Result<(), AppError> {
        if self
            .providers
            .verification_code_store
            .is_rate_limited(&email)
            .await?
        {
            return Err(AuthenticationError::TooManyRequests.into());
        }

        let verification_data = VerificationData::new();
        self.providers
            .verification_code_store
            .save(&email, &verification_data)
            .await?;

        let body = format!(
            "LiveJukeをご利用いただきありがとうございます。 認証コード: {}",
            &verification_data.code
        );

        self.providers
            .email_sender
            .send(&email, "認証コード", body)
            .await?;

        self.providers
            .verification_code_store
            .increment_rate_limit(&email)
            .await?;

        Ok(())
    }
    pub async fn verify_code(
        &self,
        email: Email,
        code: String,
        device_info: DeviceInfo,
    ) -> Result<AuthResult, AppError> {
        let Some(data) = self.providers.verification_code_store.find(&email).await? else {
            return Err(AuthenticationError::InvalidVerificationCode.into());
        };

        if data.code != code {
            self.providers
                .verification_code_store
                .increment_attempts(&email)
                .await?;
            if self
                .providers
                .verification_code_store
                .is_max_attempts(&email)
                .await?
            {
                self.providers
                    .verification_code_store
                    .delete(&email)
                    .await?
            }
            return Err(AuthenticationError::InvalidVerificationCode.into());
        }

        let authentication = self
            .repos
            .auth_repo
            .find_by_provider_uid(Provider::Email, email.as_ref())
            .await?;

        let user = if let Some(authentication) = authentication {
            self.repos
                .user_repo
                .find_by_id(authentication.user_id)
                .await?
                .ok_or(AuthenticationError::AuthenticationFailed)?
        } else {
            let new_user = NewUser::new(email.as_ref());
            let new_authentication = NewAuthentication::new(Provider::Email, email.as_ref(), None);
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

        self.providers
            .verification_code_store
            .delete(&email)
            .await?;

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
            let new_user = NewUser::new(&user_info.email);
            let new_authentication = NewAuthentication::new(Provider::Google, &user_info.sub, None);
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
