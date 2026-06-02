use async_trait::async_trait;

use crate::{
    application::release_group::import::ReleaseGroupSummary,
    domain::{artist::model::Artist, mbid::Mbid},
};

#[async_trait]
pub trait ReleaseGroupFetcher: Send + Sync {
    async fn fetch_release_groups_by_artist(
        &self,
        mbid: Mbid<Artist>,
    ) -> Result<Vec<ReleaseGroupSummary>, anyhow::Error>;
}
