use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::{artist::model::Artist, id::Id, mbid::Mbid};

#[derive(FromRow)]
pub struct ArtistRow {
    pub id: i64,
    pub mbid: Option<Uuid>,
    pub name: String,
    pub spotify_id: Option<String>,
    pub source: String,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by_user_id: Option<i64>,
    pub created_by_user_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_user_id: Option<i64>,
    pub deletion_reason: Option<String>,
}

impl TryFrom<ArtistRow> for Artist {
    type Error = anyhow::Error;

    fn try_from(value: ArtistRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            mbid: value.mbid.map(Mbid::new),
            name: value.name,
            source: value.source.parse()?,
            spotify_id: value.spotify_id,
            approved_at: value.approved_at,
            approved_by_user_id: value.approved_by_user_id.map(Id::new),
            created_by_user_id: value.created_by_user_id.map(Id::new),
            created_at: value.created_at,
            updated_at: value.updated_at,
            deleted_at: value.deleted_at,
            deleted_by_user_id: value.deleted_by_user_id.map(Id::new),
            deletion_reason: value.deletion_reason,
        })
    }
}
