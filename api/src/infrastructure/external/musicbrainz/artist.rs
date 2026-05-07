use async_trait::async_trait;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    application::{
        artist::import::{ArtistCandidate, ArtistDetail},
        traits::artist_fetcher::ArtistFetcher,
    },
    domain::{artist::model::Artist, mbid::Mbid},
    infrastructure::external::musicbrainz::http::MbClient,
};

#[derive(Debug, Deserialize)]
struct MbArtistResponse {
    artists: Vec<MbArtistDto>,
}

#[derive(Debug, Deserialize)]
struct MbArtistDto {
    id: Uuid,
    name: String,
    score: u8,
    #[serde(rename = "type")]
    artist_type: Option<MbArtistType>,
}

#[derive(Debug, Deserialize, PartialEq)]
enum MbArtistType {
    Group,
    Person,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct MbUrl {
    resource: String,
}

#[derive(Debug, Deserialize, PartialEq)]
enum MbRelationType {
    #[serde(rename = "free streaming")]
    FreeStreaming,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize)]
struct MbArtistDetailResponse {
    id: Uuid,
    name: String,
    relations: Vec<MbArtistRelationDto>,
}

#[derive(Debug, Deserialize)]
struct MbArtistRelationDto {
    url: MbUrl,
    #[serde(rename = "type")]
    relation_type: MbRelationType,
}

#[async_trait]
impl ArtistFetcher for MbClient {
    async fn search_artist_candidate(
        &self,
        name: &str,
    ) -> Result<Option<ArtistCandidate>, anyhow::Error> {
        let response: MbArtistResponse = self
            .get_json("artist", &[("query", name), ("fmt", "json")])
            .await?;

        let selected = response
            .artists
            .into_iter()
            .find(|a| {
                a.score >= 90
                    && a.artist_type
                        .as_ref()
                        .is_some_and(|t| matches!(t, MbArtistType::Group | MbArtistType::Person))
            })
            .map(|dto| ArtistCandidate {
                id: Mbid::new(dto.id),
                name: dto.name,
            });

        Ok(selected)
    }
    async fn fetch_artist_with_relations(
        &self,
        mbid: Mbid<Artist>,
    ) -> Result<ArtistDetail, anyhow::Error> {
        let path = format!("artist/{}", mbid.get());

        let response: MbArtistDetailResponse = self
            .get_json(&path, &[("inc", "url-rels"), ("fmt", "json")])
            .await?;

        let spotify_id = response
            .relations
            .into_iter()
            .find(|r| {
                r.relation_type == MbRelationType::FreeStreaming
                    && r.url.resource.contains("spotify")
            })
            .and_then(|r| {
                r.url
                    .resource
                    .rsplit('/')
                    .find(|s| !s.is_empty())
                    .map(String::from)
            });

        Ok(ArtistDetail {
            id: Mbid::new(response.id),
            name: response.name,
            spotify_id,
        })
    }
}
