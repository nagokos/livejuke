use std::str::FromStr;

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{
    application::{
        release::import::{DiscSummary, RecordingSummary, TrackSummary},
        traits::release_fetcher::ReleaseFethcer,
    },
    domain::{canonical_release::model::CanonicalRelease, mbid::Mbid},
    infrastructure::external::musicbrainz::http::MbClient,
};

#[derive(Debug, Deserialize)]
struct MbMediaDto {
    tracks: Vec<MbTrackDto>,
    position: i16,
}

impl From<MbMediaDto> for DiscSummary {
    fn from(value: MbMediaDto) -> Self {
        Self {
            position: value.position,
            tracks: value.tracks.into_iter().map(TrackSummary::from).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct MbTrackDto {
    id: Uuid,
    position: i16,
    recording: MbRecordingDto,
}

impl From<MbTrackDto> for TrackSummary {
    fn from(value: MbTrackDto) -> Self {
        Self {
            mbid: Mbid::new(value.id),
            position: value.position,
            recording: value.recording.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct MbRecordingDto {
    id: Uuid,
    title: String,
    #[serde(
        rename = "first-release-date",
        deserialize_with = "complete_date_or_none",
        default
    )]
    first_release_date: Option<NaiveDate>,
}

impl From<MbRecordingDto> for RecordingSummary {
    fn from(value: MbRecordingDto) -> Self {
        Self {
            mbid: Mbid::new(value.id),
            title: value.title,
            first_release_date: value.first_release_date,
        }
    }
}

fn complete_date_or_none<'de, D>(de: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => {
            if NaiveDate::parse_from_str(s, "%Y-%m-%d").is_err() {
                return Ok(None);
            }
            NaiveDate::from_str(s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Debug, Deserialize)]
struct MbMediaResponse {
    media: Vec<MbMediaDto>,
}

#[async_trait]
impl ReleaseFethcer for MbClient {
    async fn fetch_release_discs(
        &self,
        mbid: Mbid<CanonicalRelease>,
    ) -> Result<Vec<DiscSummary>, anyhow::Error> {
        let path = format!("release/{}", mbid.get());
        let response: MbMediaResponse = self
            .get_json(&path, &[("inc", "recordings"), ("fmt", "json")])
            .await?;

        Ok(response.media.into_iter().map(DiscSummary::from).collect())
    }
}
