use async_trait::async_trait;
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

use crate::domain::{
    canonical_release::{model::CanonicalRelease, repository::CanonicalReleaseRepository},
    mbid::Mbid,
    release_group::model::ReleaseGroup,
};

pub struct PgCanonicalReleaseRepository {
    pool: PgPool,
}

impl PgCanonicalReleaseRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, FromRow)]
struct ReleaseGroupRow {
    canonical_release_mbid: Uuid,
}

#[async_trait]
impl CanonicalReleaseRepository for PgCanonicalReleaseRepository {
    async fn find_canonical_release_mbid(
        &self,
        uuid: Mbid<ReleaseGroup>,
    ) -> Result<Mbid<CanonicalRelease>, anyhow::Error> {
        let sql = r#"
            SELECT 
                canonical_release_mbid
            FROM canonical_releases 
            WHERE release_group_mbid = $1
            LIMIT 1
        "#;

        let row = sqlx::query_as::<_, ReleaseGroupRow>(sql)
            .bind(uuid.get())
            .fetch_one(&self.pool)
            .await?;

        Ok(Mbid::new(row.canonical_release_mbid))
    }
}
