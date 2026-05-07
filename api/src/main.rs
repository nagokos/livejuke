use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use axum::{Json, http::StatusCode, response::IntoResponse};
use sqlx::postgres::PgPoolOptions;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    application::{
        artist::service::{ArtistProviders, ArtistRepositories, ArtistService},
        auth::{
            config::AuthConfig,
            service::{AuthProviders, AuthRepositories, AuthService},
        },
        release_group::service::{
            ReleaseGroupProviders, ReleaseGroupRepositories, ReleaseGroupService,
        },
        traits::access_token_provider::AccessTokenProvider,
        user::service::{UserProviders, UserRepositories, UserService},
    },
    config::Config,
    infrastructure::{
        auth::{
            jwt_access_token_provider::JwtAccessTokenProvider,
            opaque_refresh_token_provider::OpaqueRefreshTokenProvider,
        },
        clock::SystemClock,
        external::{
            aws_s3_store::AwsS3Store,
            google_token_verifier::GoogleTokenVerifier,
            musicbrainz::{
                client::build_musicbrainz_client, http::MbClient, rate_limiter::MbRateLimiter,
            },
            redis_upload_session_store::RedisUploadSessionStore,
            redis_verification_code_store::RedisVerificationCodeStore,
            smtp_email_sender::{SmtpConfig, SmtpEmailSender},
        },
        persistence::{
            artist::repository::PgArtistRepository,
            pg_authentication_repository::PgAuthenticationRepository,
            pg_canonical_release_repository::PgCanonicalReleaseRepository,
            pg_session_repository::PgSessionRepository, pg_user_repository::PgUserRepository,
            release_group::repository::PgReleaseGroupRepository,
        },
    },
    presentation::{
        error::ErrorResponse,
        error_code::ErrorCode,
        handlers::{
            auth::{create_private_auth_router, create_public_auth_router},
            user::create_user_router,
        },
        middleware::auth::auth_middleware,
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
    user_service: Arc<UserService>,
    artist_service: Arc<ArtistService>,
    release_group_service: Arc<ReleaseGroupService>,
    access_token_provider: Arc<dyn AccessTokenProvider>,
    resend_cooldown_seconds: u8,
    cdn_base_url: String,
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

    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));

    let access_token_provider = Arc::new(JwtAccessTokenProvider::new(
        config.access_token_secret,
        config.access_token_exp_secs,
    ));

    let email_sender = Arc::new(SmtpEmailSender::try_new(SmtpConfig {
        host: config.smtp_host,
        port: config.smtp_port,
        username: config.smtp_username,
        password: config.smtp_password,
        from: config.smtp_from,
        tls: config.smtp_tls,
    })?);

    let verification_code_store = Arc::new(RedisVerificationCodeStore::new(
        redis_conn.clone(),
        config.verification_code_exp_secs,
        config.max_attempts,
        config.max_attempts_ttl_secs,
        config.rate_limit,
        config.rate_limit_ttl_secs,
    ));

    let clock = Arc::new(SystemClock);

    let http_client = reqwest::Client::new();

    let mb_client = {
        let http_client =
            build_musicbrainz_client("LiveJuke", env!("CARGO_PKG_VERSION"), &config.contact_url)?;
        let rate_limiter = MbRateLimiter::new();

        Arc::new(MbClient::new(http_client, rate_limiter))
    };

    let auth_service = {
        let repositories = AuthRepositories {
            auth_repo: Arc::new(PgAuthenticationRepository::new(pool.clone())),
            user_repo: user_repo.clone(),
            session_repo: Arc::new(PgSessionRepository::new(pool.clone())),
        };
        let providers = AuthProviders {
            access_token_provider: access_token_provider.clone(),
            refresh_token_provider: Arc::new(OpaqueRefreshTokenProvider),
            id_token_verifier: Arc::new(
                GoogleTokenVerifier::new(config.google_client_id, http_client.clone()).await?,
            ),
            verification_code_store: verification_code_store.clone(),
            email_sender: email_sender.clone(),
        };
        let config = AuthConfig::new(config.refresh_token_exp_secs);

        Arc::new(AuthService::new(repositories, providers, config))
    };

    let user_service = {
        let repositories = UserRepositories {
            user_repo: user_repo.clone(),
        };
        let providers = UserProviders {
            object_store: Arc::new(AwsS3Store::new(config.aws_s3_bucket_name).await),
            upload_session_store: Arc::new(RedisUploadSessionStore::new(redis_conn.clone())),
            verification_code_store: verification_code_store.clone(),
            email_sender: email_sender.clone(),
        };
        Arc::new(UserService::new(repositories, providers))
    };

    let artist_service = {
        let repositories = ArtistRepositories {
            artist_repo: Arc::new(PgArtistRepository::new(pool.clone())),
        };
        let providers = ArtistProviders {
            artist_fetcher: mb_client.clone(),
        };
        Arc::new(ArtistService::new(repositories, providers, clock.clone()))
    };

    let release_group_service = {
        let repositories = ReleaseGroupRepositories {
            release_group_repo: Arc::new(PgReleaseGroupRepository::new(pool.clone())),
            canonical_release_repo: Arc::new(PgCanonicalReleaseRepository::new(pool.clone())),
        };
        let providers = ReleaseGroupProviders {
            release_group_fetcher: mb_client.clone(),
            release_fetcher: mb_client.clone(),
        };
        Arc::new(ReleaseGroupService::new(
            repositories,
            providers,
            clock.clone(),
        ))
    };

    let app_state = AppState {
        auth_service,
        user_service,
        artist_service,
        release_group_service,
        access_token_provider,
        resend_cooldown_seconds: config.resend_cooldown_secs,
        cdn_base_url: config.cdn_base_url,
    };

    let (public_router, public_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/auth", create_public_auth_router())
        .split_for_parts();

    let (private_router, private_api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/me", create_user_router())
        .nest("/auth", create_private_auth_router())
        .layer(axum::middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .split_for_parts();

    let openapi = public_api.merge_from(private_api);

    let router = public_router
        .merge(private_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .layer(GovernorLayer::new(governor_conf).error_handler(|_| {
            let error_response = ErrorResponse {
                code: ErrorCode::GlobalRateLimited,
                message: "too many requests".to_string(),
            };
            (StatusCode::TOO_MANY_REQUESTS, Json(error_response)).into_response()
        }))
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
