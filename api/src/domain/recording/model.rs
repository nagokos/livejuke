use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::{id::Id, mbid::Mbid, user::model::User};

#[derive(Debug)]
pub struct Recording {
    pub id: Id<Recording>,
    pub mbid: Option<Mbid<Recording>>,
    pub title: String,
    pub first_release_date: Option<NaiveDate>,
    pub source: RecordingSource,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by_user_id: Option<Id<User>>,
    pub created_by_user_id: Option<Id<User>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub deleted_by_user_id: Option<Id<User>>,
    pub deletion_reason: Option<String>,
}

#[derive(Debug)]
pub enum RecordingSource {
    MusicBrainz,
    Manual,
}

impl RecordingSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MusicBrainz => "musicbrainz",
            Self::Manual => "manual",
        }
    }
}

impl FromStr for RecordingSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "musicbrainz" => Ok(Self::MusicBrainz),
            "manual" => Ok(Self::Manual),
            _ => Err(anyhow::anyhow!("unknown recording source: {}", s)),
        }
    }
}
