use async_trait::async_trait;

use crate::domain::{
    artist::model::Artist,
    id::Id,
    release_group::payload::{ImportFromMbReleaseGroupPayload, ImportFromMbTrackPayload},
};

#[async_trait]
pub trait ReleaseGroupRepository: Send + Sync {
    async fn import_from_mb(
        &self,
        artist_id: Id<Artist>,
        release_group: ImportFromMbReleaseGroupPayload,
        tracks: Vec<ImportFromMbTrackPayload>,
    ) -> Result<(), anyhow::Error>;
}
