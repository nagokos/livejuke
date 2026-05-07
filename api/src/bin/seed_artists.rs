use std::sync::Arc;

use api::{
    application::{
        artist::service::{ArtistProviders, ArtistRepositories, ArtistService},
        release_group::service::{
            ReleaseGroupProviders, ReleaseGroupRepositories, ReleaseGroupService,
        },
    },
    config::Config,
    infrastructure::{
        clock::SystemClock,
        external::musicbrainz::{
            client::build_musicbrainz_client, http::MbClient, rate_limiter::MbRateLimiter,
        },
        persistence::{
            artist::repository::PgArtistRepository,
            pg_canonical_release_repository::PgCanonicalReleaseRepository,
            release_group::repository::PgReleaseGroupRepository,
        },
    },
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let clock = Arc::new(SystemClock);

    let mb_client = {
        let http_client =
            build_musicbrainz_client("LiveJuke", env!("CARGO_PKG_VERSION"), &config.contact_url)?;
        let rate_limiter = MbRateLimiter::new();

        Arc::new(MbClient::new(http_client, rate_limiter))
    };

    let artist_service = {
        let repositories = ArtistRepositories {
            artist_repo: Arc::new(PgArtistRepository::new(pool.clone())),
        };
        let providers = ArtistProviders {
            artist_fetcher: mb_client.clone(),
        };
        ArtistService::new(repositories, providers, clock.clone())
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

    let artists = artist_service.seed().await?;

    for artist in artists {
        release_group_service.seed(artist).await?;
    }

    println!("Successfully inserted initial data🎉");

    Ok(())
}
