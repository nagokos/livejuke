use std::sync::Arc;

use axum::Router;
use sqlx::postgres::PgPoolOptions;

use crate::{
    application::services::auth_service::AuthService,
    config::Config,
    infrastructure::persistence::{pg_auth_repo::PgAuthRepo, pg_user_repo::PgUserRepo},
    presentation::handlers::auth::create_auth_router,
};

pub mod config;
pub mod error;

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

#[derive(Clone)]
pub struct AppState {
    auth_service: Arc<AuthService<PgAuthRepo, PgUserRepo>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api=debug,axum=info,tower_http=debug".into()),
        )
        .init();

    let config = Config::from_env()?;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let user_repo = PgUserRepo::new(pool.clone());
    let auth_repo = PgAuthRepo::new(pool.clone());
    let auth_service = Arc::new(AuthService::new(auth_repo, user_repo));

    let app_state = AppState { auth_service };

    let auth_router = create_auth_router();
    let router = Router::new().merge(auth_router).with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await?;

    Ok(())
}
