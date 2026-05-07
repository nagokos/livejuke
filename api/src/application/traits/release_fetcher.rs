use async_trait::async_trait;

use crate::{
    application::release::import::DiscSummary,
    domain::{canonical_release::model::CanonicalRelease, mbid::Mbid},
};

#[async_trait]
pub trait ReleaseFethcer: Send + Sync {
    async fn fetch_release_discs(
        &self,
        mbid: Mbid<CanonicalRelease>,
    ) -> Result<Vec<DiscSummary>, anyhow::Error>;
}
