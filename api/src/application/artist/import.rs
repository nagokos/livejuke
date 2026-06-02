use crate::domain::{artist::model::Artist, mbid::Mbid};

#[derive(Debug)]
pub struct ArtistCandidate {
    pub id: Mbid<Artist>,
    pub name: String,
}

#[derive(Debug)]
pub struct ArtistDetail {
    pub id: Mbid<Artist>,
    pub name: String,
    pub spotify_id: Option<String>,
}
