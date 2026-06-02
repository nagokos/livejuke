use chrono::{DateTime, NaiveDate, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::{id::Id, mbid::Mbid, release_group::model::ReleaseGroup};

#[derive(FromRow)]
pub struct ReleaseGroupRow {
    pub id: i64,
    pub artist_id: i64,
    pub mbid: Option<Uuid>,
    pub title: String,
    pub primary_type: String,
    pub release_date: Option<NaiveDate>,
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

impl TryFrom<ReleaseGroupRow> for ReleaseGroup {
    type Error = anyhow::Error;

    fn try_from(value: ReleaseGroupRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            artist_id: Id::new(value.artist_id),
            mbid: value.mbid.map(Mbid::new),
            title: value.title,
            source: value.source.parse()?,
            primary_type: value.primary_type.parse()?,
            release_date: value.release_date,
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
