use async_trait::async_trait;

use crate::domain::{
    canonical_release::model::CanonicalRelease, mbid::Mbid, release_group::model::ReleaseGroup,
};

#[async_trait]
pub trait CanonicalReleaseRepository: Send + Sync {
    async fn find_canonical_release_mbid(
        &self,
        uuid: Mbid<ReleaseGroup>,
    ) -> Result<Mbid<CanonicalRelease>, anyhow::Error>;
}
