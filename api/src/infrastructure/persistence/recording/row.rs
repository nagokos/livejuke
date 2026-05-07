use chrono::{DateTime, NaiveDate, Utc};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::domain::{id::Id, mbid::Mbid, recording::model::Recording};

#[derive(FromRow)]
struct RecordingRow {
    id: i64,
    mbid: Option<Uuid>,
    title: String,
    first_release_date: Option<NaiveDate>,
    source: String,
    approved_at: Option<DateTime<Utc>>,
    approved_by_user_id: Option<i64>,
    created_by_user_id: Option<i64>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
    deleted_by_user_id: Option<i64>,
    deletion_reason: Option<String>,
}

impl TryFrom<RecordingRow> for Recording {
    type Error = anyhow::Error;

    fn try_from(value: RecordingRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: Id::new(value.id),
            mbid: value.mbid.map(Mbid::new),
            title: value.title,
            first_release_date: value.first_release_date,
            source: value.source.parse()?,
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

#[derive(FromRow)]
pub struct RecordingIdRow {
    pub id: i64,
    pub mbid: Uuid,
}
