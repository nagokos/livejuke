use std::sync::Arc;

use crate::{
    application::traits::{artist_fetcher::ArtistFetcher, clock::Clock},
    domain::artist::{model::Artist, payload::ImportArtistPayload, repository::ArtistRepository},
};

pub struct ArtistRepositories {
    pub artist_repo: Arc<dyn ArtistRepository>,
}

pub struct ArtistProviders {
    pub artist_fetcher: Arc<dyn ArtistFetcher>,
}

pub struct ArtistService {
    repos: ArtistRepositories,
    providers: ArtistProviders,
    clock: Arc<dyn Clock>,
}

impl ArtistService {
    pub fn new(
        repos: ArtistRepositories,
        providers: ArtistProviders,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            repos,
            providers,
            clock,
        }
    }
    // pub async fn import_artist_by_name(&self, name: &str) -> Result<Artist, anyhow::Error> {
    //     let candidates = self.providers.artist_fetcher.search_artist(name).await?;
    //
    //     let Some(selected) = candidates.into_iter().find(|artist| {
    //         artist.score >= 90
    //             && artist
    //                 .artist_type
    //                 .as_ref()
    //                 .is_some_and(|t| *t == ArtistType::Group || *t == ArtistType::Person)
    //     }) else {
    //         println!("No results found. Please register the artist.{}", name);
    //         return Err(anyhow::anyhow!("error"));
    //     };
    //
    //     let detail = self
    //         .providers
    //         .artist_fetcher
    //         .fetch_artist_with_relations(selected.id)
    //         .await?;
    //
    //     let now = self.clock.now();
    //     let payload = ImportArtistPayload::new(detail.id, detail.name, detail.spotify_id, now);
    // }
    pub async fn seed(&self) -> Result<Vec<Artist>, anyhow::Error> {
        let names = [
            "Mr.Children",
            "SUPER BEAVER",
            "Vaundy",
            "レミオロメン",
            "sumika",
            "RADWIMPS",
            "あいみょん",
            "Mrs. GREEN APPLE",
        ];
        let mut seed_artists = Vec::new();

        for name in names {
            let Some(selected) = self
                .providers
                .artist_fetcher
                .search_artist_candidate(name)
                .await?
            else {
                tracing::warn!(artist_name = name, "no candidate found");
                return Err(anyhow::anyhow!("not artist candidate found"));
            };

            println!("selected artist {}", selected.name);

            let detail = self
                .providers
                .artist_fetcher
                .fetch_artist_with_relations(selected.id)
                .await?;

            let now = self.clock.now();
            let payload = ImportArtistPayload::new(detail.id, detail.name, detail.spotify_id, now);

            seed_artists.push(payload);
        }
        let artists = self.repos.artist_repo.bulk_insert(seed_artists).await?;

        Ok(artists)
    }
}
