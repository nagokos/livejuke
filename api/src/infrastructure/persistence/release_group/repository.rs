use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::{
    domain::{
        artist::model::Artist,
        id::Id,
        release_group::{
            payload::{ImportFromMbReleaseGroupPayload, ImportFromMbTrackPayload},
            repository::ReleaseGroupRepository,
        },
    },
    infrastructure::persistence::recording::row::RecordingIdRow,
};

pub struct PgReleaseGroupRepository {
    pool: PgPool,
}

impl PgReleaseGroupRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReleaseGroupRepository for PgReleaseGroupRepository {
    async fn import_from_mb(
        &self,
        artist_id: Id<Artist>,
        release_group: ImportFromMbReleaseGroupPayload,
        tracks: Vec<ImportFromMbTrackPayload>,
    ) -> Result<(), anyhow::Error> {
        let mut tx = self.pool.begin().await?;

        let sql = r#"
            INSERT INTO release_groups (
                artist_id,
                mbid,
                title,
                primary_type,
                release_date,
                source,
                approved_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING
                id
        "#;
        let release_group_id = sqlx::query_scalar::<_, i64>(sql)
            .bind(artist_id.get())
            .bind(release_group.mbid.get())
            .bind(release_group.title)
            .bind(release_group.primary_type.as_str())
            .bind(release_group.release_date)
            .bind(release_group.source.as_str())
            .bind(release_group.approved_at)
            .fetch_one(&mut *tx)
            .await?;

        if !release_group.secondary_types.is_empty() {
            let sql = r#"
            INSERT INTO release_group_secondary_types (
                release_group_id,
                secondary_type
            )
        "#;
            let mut query_builder = QueryBuilder::new(sql);
            query_builder.push_values(release_group.secondary_types, |mut b, secondary_type| {
                b.push_bind(release_group_id)
                    .push_bind(secondary_type.as_str().to_string());
            });
            query_builder.build().execute(&mut *tx).await?;
        }

        let sql = r#"
            INSERT INTO recordings (
                mbid,
                title,
                first_release_date,
                source,
                approved_at
            )
        "#;
        let mut query_builder = QueryBuilder::new(sql);
        query_builder.push_values(tracks.iter(), |mut b, track| {
            b.push_bind(track.recording.mbid.get())
                .push_bind(&track.recording.title)
                .push_bind(track.recording.first_release_date)
                .push_bind(track.recording.source.as_str().to_string())
                .push_bind(track.recording.approved_at);
        });
        query_builder.push(" ON CONFLICT (mbid) DO UPDATE SET title = EXCLUDED.title");
        query_builder.push(" RETURNING id, mbid");
        let recordings = query_builder
            .build_query_as::<RecordingIdRow>()
            .fetch_all(&mut *tx)
            .await?;

        let recording_hash: HashMap<Uuid, i64> = recordings
            .into_iter()
            .map(|recording| (recording.mbid, recording.id))
            .collect();

        let sql = r#"
            INSERT INTO tracks (
                release_group_id,
                recording_id,
                mbid,
                disc_number,
                track_number
            )
        "#;
        let mut query_builder = QueryBuilder::new(sql);
        query_builder.push_values(tracks.iter(), |mut b, track| {
            let recording_id = recording_hash.get(&track.recording.mbid.get()).unwrap();
            b.push_bind(release_group_id)
                .push_bind(recording_id)
                .push_bind(track.mbid.get())
                .push_bind(track.disc_number)
                .push_bind(track.track_number);
        });
        query_builder.build().execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(())
    }
}
