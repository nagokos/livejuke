use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::Ok;
use sqlx::postgres::PgPoolOptions;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    application::auth::service::AuthService,
    config::Config,
    infrastructure::{
        auth::{argon2_hasher::Argon2Hasher, jwt_token_provider::JwtTokenProvider},
        persistence::{
            pg_authentication_repository::PgAuthenticationRepository,
            pg_user_repository::PgUserRepository,
        },
    },
    presentation::{cookie::CookieConfig, handlers::auth::create_auth_router},
};

pub mod config;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

type ConcreteAuthService =
    AuthService<PgAuthenticationRepository, PgUserRepository, Argon2Hasher, JwtTokenProvider>;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<ConcreteAuthService>,
    cookie_config: Arc<CookieConfig>,
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

    let cookie_config = Arc::new(CookieConfig::from_env(config.app_env));

    let user_repo = PgUserRepository::new(pool.clone());
    let auth_repo = PgAuthenticationRepository::new(pool.clone());
    let token_provider = JwtTokenProvider::new(config.jwt_secret, config.jwt_expiration);
    let auth_service = Arc::new(AuthService::new(
        auth_repo,
        user_repo,
        Argon2Hasher,
        token_provider,
    ));

    let app_state = AppState {
        auth_service,
        cookie_config,
    };

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/auth", create_auth_router())
        .split_for_parts();
    let router = router
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api))
        .layer(GovernorLayer::new(governor_conf))
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
