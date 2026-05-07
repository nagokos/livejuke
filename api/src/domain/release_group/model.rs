use std::str::FromStr;

use chrono::{DateTime, NaiveDate, Utc};

use crate::domain::{artist::model::Artist, id::Id, mbid::Mbid, user::model::User};

#[derive(Debug)]
pub struct ReleaseGroup {
    pub id: Id<ReleaseGroup>,
    pub artist_id: Id<Artist>,
    pub mbid: Option<Mbid<ReleaseGroup>>,
    pub title: String,
    pub primary_type: PrimaryType,
    pub release_date: Option<NaiveDate>,
    pub source: ReleaseGroupSource,
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
pub enum PrimaryType {
    Single,
    Album,
    Ep,
}

impl PrimaryType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Single => "single",
            Self::Album => "album",
            Self::Ep => "ep",
        }
    }
}

impl FromStr for PrimaryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Self::Single),
            "album" => Ok(Self::Album),
            "ep" => Ok(Self::Ep),
            _ => Err(anyhow::anyhow!("unknown primary_type: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum ReleaseGroupSource {
    MusicBrainz,
    Manual,
}

impl ReleaseGroupSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MusicBrainz => "musicbrainz",
            Self::Manual => "manual",
        }
    }
}

impl FromStr for ReleaseGroupSource {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "musicbrainz" => Ok(Self::MusicBrainz),
            "manual" => Ok(Self::Manual),
            _ => Err(anyhow::anyhow!("unknown recording source: {}", s)),
        }
    }
}

#[derive(Debug)]
pub enum SecondaryType {
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    AudioDrama,
    Live,
    Remix,
    DjMix,
    MixtapeStreet,
    Demo,
    FieldRecording,
}

impl SecondaryType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Compilation => "compilation",
            Self::Soundtrack => "soundtrack",
            Self::Spokenword => "spokenword",
            Self::Interview => "interview",
            Self::Audiobook => "audiobook",
            Self::AudioDrama => "audio_drama",
            Self::Live => "live",
            Self::Remix => "remix",
            Self::DjMix => "dj_mix",
            Self::MixtapeStreet => "mixtape_street",
            Self::Demo => "demo",
            Self::FieldRecording => "field_recording",
        }
    }
}

impl FromStr for SecondaryType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compilation" => Ok(Self::Compilation),
            "soundtrack" => Ok(Self::Soundtrack),
            "spokenword" => Ok(Self::Spokenword),
            "interview" => Ok(Self::Interview),
            "audiobook" => Ok(Self::Audiobook),
            "audio_drama" => Ok(Self::AudioDrama),
            "live" => Ok(Self::Live),
            "remix" => Ok(Self::Remix),
            "dj_mix" => Ok(Self::DjMix),
            "mixtape_street" => Ok(Self::MixtapeStreet),
            "demo" => Ok(Self::Demo),
            "field_recording" => Ok(Self::FieldRecording),
            _ => Err(anyhow::anyhow!("unknown secondary type: {}", s)),
        }
    }
}
