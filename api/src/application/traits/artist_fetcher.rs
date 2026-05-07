use async_trait::async_trait;

use crate::{
    application::artist::import::{ArtistCandidate, ArtistDetail},
    domain::{artist::model::Artist, mbid::Mbid},
};

#[async_trait]
pub trait ArtistFetcher: Send + Sync {
    async fn search_artist_candidate(
        &self,
        name: &str,
    ) -> Result<Option<ArtistCandidate>, anyhow::Error>;
    async fn fetch_artist_with_relations(
        &self,
        mbid: Mbid<Artist>,
    ) -> Result<ArtistDetail, anyhow::Error>;
}
