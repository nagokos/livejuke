use std::sync::Arc;

use crate::{
    application::traits::{
        clock::Clock, release_fetcher::ReleaseFethcer, release_group_fetcher::ReleaseGroupFetcher,
    },
    domain::{
        artist::model::Artist,
        canonical_release::repository::CanonicalReleaseRepository,
        release_group::{
            payload::{
                ImportFromMbRecordingPayload, ImportFromMbReleaseGroupPayload,
                ImportFromMbTrackPayload,
            },
            repository::ReleaseGroupRepository,
        },
    },
};

pub struct ReleaseGroupRepositories {
    pub release_group_repo: Arc<dyn ReleaseGroupRepository>,
    pub canonical_release_repo: Arc<dyn CanonicalReleaseRepository>,
}

pub struct ReleaseGroupProviders {
    pub release_group_fetcher: Arc<dyn ReleaseGroupFetcher>,
    pub release_fetcher: Arc<dyn ReleaseFethcer>,
}

pub struct ReleaseGroupService {
    pub repos: ReleaseGroupRepositories,
    pub providers: ReleaseGroupProviders,
    pub clock: Arc<dyn Clock>,
}

impl ReleaseGroupService {
    pub fn new(
        repos: ReleaseGroupRepositories,
        providers: ReleaseGroupProviders,
        clock: Arc<dyn Clock>,
    ) -> Self {
        Self {
            repos,
            providers,
            clock,
        }
    }
    pub async fn seed(&self, artist: Artist) -> Result<(), anyhow::Error> {
        let release_groups = self
            .providers
            .release_group_fetcher
            .fetch_release_groups_by_artist(artist.mbid.unwrap())
            .await?;

        for release_group in release_groups {
            let mbid = self
                .repos
                .canonical_release_repo
                .find_canonical_release_mbid(release_group.id)
                .await?;

            let discs = self
                .providers
                .release_fetcher
                .fetch_release_discs(mbid)
                .await?;

            let now = self.clock.now();
            let release_group_payload = ImportFromMbReleaseGroupPayload::new(
                release_group.title,
                release_group.id,
                release_group.primary_type,
                release_group.release_date,
                now,
                release_group.secondary_types,
            );
            let tracks_payload = discs
                .into_iter()
                .flat_map(|disc| {
                    disc.tracks
                        .into_iter()
                        .map(|track| ImportFromMbTrackPayload {
                            mbid: track.mbid,
                            disc_number: disc.position,
                            track_number: track.position,
                            recording: ImportFromMbRecordingPayload::new(
                                track.recording.mbid,
                                track.recording.title,
                                track.recording.first_release_date,
                                now,
                            ),
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            self.repos
                .release_group_repo
                .import_from_mb(artist.id, release_group_payload, tracks_payload)
                .await?;
        }

        Ok(())
    }
}
