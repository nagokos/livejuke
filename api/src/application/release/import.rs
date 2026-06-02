use chrono::NaiveDate;

use crate::domain::{mbid::Mbid, recording::model::Recording, track::model::Track};

#[derive(Debug)]
pub struct DiscSummary {
    pub position: i16,
    pub tracks: Vec<TrackSummary>,
}

#[derive(Debug)]
pub struct TrackSummary {
    pub mbid: Mbid<Track>,
    pub position: i16,
    pub recording: RecordingSummary,
}

#[derive(Debug)]
pub struct RecordingSummary {
    pub mbid: Mbid<Recording>,
    pub title: String,
    pub first_release_date: Option<NaiveDate>,
}
