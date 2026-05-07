use async_trait::async_trait;

use crate::domain::artist::{model::Artist, payload::ImportArtistPayload};

#[async_trait]
pub trait ArtistRepository: Send + Sync {
    async fn bulk_insert(
        &self,
        artists: Vec<ImportArtistPayload>,
    ) -> Result<Vec<Artist>, anyhow::Error>;
}
