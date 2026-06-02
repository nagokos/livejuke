use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::{
    mbid::Mbid,
    recording::model::{Recording, RecordingSource},
    release_group::model::{PrimaryType, ReleaseGroup, ReleaseGroupSource, SecondaryType},
    track::model::Track,
};

#[derive(Debug)]
pub struct ImportFromMbReleaseGroupPayload {
    pub title: String,
    pub mbid: Mbid<ReleaseGroup>,
    pub primary_type: PrimaryType,
    pub source: ReleaseGroupSource,
    pub release_date: Option<NaiveDate>,
    pub approved_at: DateTime<Utc>,
    pub secondary_types: Vec<SecondaryType>,
}

impl ImportFromMbReleaseGroupPayload {
    pub fn new(
        title: String,
        mbid: Mbid<ReleaseGroup>,
        primary_type: PrimaryType,
        release_date: Option<NaiveDate>,
        approved_at: DateTime<Utc>,
        secondary_types: Vec<SecondaryType>,
    ) -> Self {
        Self {
            title,
            mbid,
            primary_type,
            source: ReleaseGroupSource::MusicBrainz,
            release_date,
            approved_at,
            secondary_types,
        }
    }
}

#[derive(Debug)]
pub struct ImportFromMbTrackPayload {
    pub mbid: Mbid<Track>,
    pub disc_number: i16,
    pub track_number: i16,
    pub recording: ImportFromMbRecordingPayload,
}

#[derive(Debug)]
pub struct ImportFromMbRecordingPayload {
    pub mbid: Mbid<Recording>,
    pub title: String,
    pub first_release_date: Option<NaiveDate>,
    pub source: RecordingSource,
    pub approved_at: DateTime<Utc>,
}

impl ImportFromMbRecordingPayload {
    pub fn new(
        mbid: Mbid<Recording>,
        title: String,
        first_release_date: Option<NaiveDate>,
        approved_at: DateTime<Utc>,
    ) -> Self {
        Self {
            mbid,
            title,
            first_release_date,
            source: RecordingSource::MusicBrainz,
            approved_at,
        }
    }
}
