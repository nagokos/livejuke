use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::Ok;
use axum::{Json, http::StatusCode, response::IntoResponse};
use sqlx::postgres::PgPoolOptions;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    application::{
        auth::{
            config::AuthConfig,
            service::{AuthProviders, AuthRepositories, AuthService},
        },
        traits::access_token_provider::AccessTokenProvider,
    },
    config::Config,
    infrastructure::{
        auth::{
            jwt_access_token_provider::JwtAccessTokenProvider,
            opaque_refresh_token_provider::OpaqueRefreshTokenProvider,
        },
        external::{
            google_token_verifier::GoogleTokenVerifier,
            smtp_email_sender::{SmtpConfig, SmtpEmailSender},
        },
        persistence::{
            pg_authentication_repository::PgAuthenticationRepository,
            pg_session_repository::PgSessionRepository, pg_user_repository::PgUserRepository,
            redis_verification_code_store::RedisVerificationCodeStore,
        },
    },
    presentation::{
        error::ErrorResponse, error_code::ErrorCode, handlers::auth::create_auth_router,
    },
};

pub mod config;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService>,
    access_token_provider: Arc<dyn AccessTokenProvider>,
    resend_cooldown_seconds: u8,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,axum=info,tower_http=debug".into()),
        )
        .init();

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(10)
        .burst_size(30)
        .finish()
        .expect("failed to build governor config");
    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            tracing::info!("rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let redis_conn = {
        let redis_client = redis::Client::open(config.redis_url)?;
        redis_client.get_multiplexed_async_connection().await?
    };

    let access_token_provider = Arc::new(JwtAccessTokenProvider::new(
        config.access_token_secret,
        config.access_token_expiration,
    ));
    let auth_service = {
        let auth_repositories = AuthRepositories {
            auth_repo: Arc::new(PgAuthenticationRepository::new(pool.clone())),
            user_repo: Arc::new(PgUserRepository::new(pool.clone())),
            session_repo: Arc::new(PgSessionRepository::new(pool.clone())),
        };
        let auth_providers = AuthProviders {
            access_token_provider: access_token_provider.clone(),
            refresh_token_provider: Arc::new(OpaqueRefreshTokenProvider),
            id_token_verifier: Arc::new(
                GoogleTokenVerifier::new(config.google_client_id, reqwest::Client::new()).await?,
            ),
            verification_code_store: Arc::new(RedisVerificationCodeStore::new(redis_conn.clone())),
            email_sender: Arc::new(SmtpEmailSender::try_new(SmtpConfig {
                host: config.smtp_host,
                port: config.smtp_port,
                username: config.smtp_username,
                password: config.smtp_password,
                from: config.smtp_from,
                tls: config.smtp_tls,
            })?),
        };
        let auth_config = AuthConfig::new(config.refresh_token_expiration);

        Arc::new(AuthService::new(
            auth_repositories,
            auth_providers,
            auth_config,
        ))
    };

    let app_state = AppState {
        auth_service,
        access_token_provider,
        resend_cooldown_seconds: config.resend_cooldown_seconds,
    };

    let (public_router, public_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/auth", create_auth_router())
        .split_for_parts();

    let router = public_router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", public_api))
        .layer(GovernorLayer::new(governor_conf).error_handler(|_| {
            let error_response = ErrorResponse {
                code: ErrorCode::RateLimitExceeded,
                message: "too many requests".to_string(),
            };
            (StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response()
        }))
        // .layer(axum::middleware::from_fn_with_state(
        //     app_state.clone(),
        //     auth_middleware,
        // ))
        .with_state(app_state);

    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 3000));
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
