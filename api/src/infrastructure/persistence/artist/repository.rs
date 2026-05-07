use async_trait::async_trait;
use sqlx::PgPool;

use crate::{
    domain::artist::{model::Artist, payload::ImportArtistPayload, repository::ArtistRepository},
    infrastructure::persistence::artist::row::ArtistRow,
};

pub struct PgArtistRepository {
    pool: PgPool,
}

impl PgArtistRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArtistRepository for PgArtistRepository {
    async fn bulk_insert(
        &self,
        artists: Vec<ImportArtistPayload>,
    ) -> Result<Vec<Artist>, anyhow::Error> {
        let sql = r#"
            INSERT INTO artists (
                mbid,
                name,
                spotify_id,
                source,
                approved_at
            )
        "#;
        let mut query_builder = sqlx::QueryBuilder::new(sql);

        query_builder.push_values(artists, |mut b, artist| {
            b.push_bind(artist.mbid.get())
                .push_bind(artist.name)
                .push_bind(artist.spotify_id)
                .push_bind(artist.source.as_str().to_string())
                .push_bind(artist.approved_at);
        });

        let sql = r#"
            RETURNING 
                id,
                mbid,
                name,
                spotify_id,
                source,
                approved_at,
                approved_by_user_id,
                created_by_user_id,
                created_at,
                updated_at,
                deleted_at,
                deleted_by_user_id,
                deletion_reason
        "#;
        query_builder.push(sql);

        let artist_rows = query_builder
            .build_query_as::<ArtistRow>()
            .fetch_all(&self.pool)
            .await?;

        let artists: Vec<Artist> = artist_rows
            .into_iter()
            .filter_map(|row| row.try_into().ok())
            .collect();

        Ok(artists)
    }
}
