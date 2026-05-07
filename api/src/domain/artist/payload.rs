use chrono::{DateTime, Utc};

use crate::domain::{
    artist::model::{Artist, ArtistSource},
    mbid::Mbid,
};

#[derive(Debug)]
pub struct ImportArtistPayload {
    pub mbid: Mbid<Artist>,
    pub name: String,
    pub source: ArtistSource,
    pub spotify_id: Option<String>,
    pub approved_at: DateTime<Utc>,
}

impl ImportArtistPayload {
    pub fn new(
        mbid: Mbid<Artist>,
        name: String,
        spotify_id: Option<String>,
        approved_at: DateTime<Utc>,
    ) -> Self {
        Self {
            mbid,
            name,
            source: ArtistSource::MusicBrainz,
            spotify_id,
            approved_at,
        }
    }
}
