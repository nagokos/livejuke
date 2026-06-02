use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use crate::{
    application::{
        release_group::import::ReleaseGroupSummary,
        traits::release_group_fetcher::ReleaseGroupFetcher,
    },
    domain::{
        artist::model::Artist,
        mbid::Mbid,
        release_group::model::{PrimaryType, SecondaryType},
    },
    infrastructure::external::musicbrainz::http::MbClient,
};

#[derive(Debug, Deserialize)]
pub struct MbReleaseGroupDto {
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "secondary-types")]
    pub secondary_types: Vec<MbSecondaryType>,
    #[serde(rename = "primary-type")]
    pub primary_type: MbPrimaryType,
    #[serde(
        rename = "first-release-date",
        deserialize_with = "complete_date_or_none",
        default
    )]
    pub release_date: Option<NaiveDate>,
}

fn complete_date_or_none<'de, D>(de: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => {
            let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") else {
                return Ok(None);
            };
            Ok(Some(date))
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum MbPrimaryType {
    Single,
    Album,
    #[serde(rename = "EP")]
    Ep,
}

impl From<MbPrimaryType> for PrimaryType {
    fn from(value: MbPrimaryType) -> Self {
        match value {
            MbPrimaryType::Single => Self::Single,
            MbPrimaryType::Album => Self::Album,
            MbPrimaryType::Ep => Self::Ep,
        }
    }
}

#[derive(Debug, Deserialize)]
pub enum MbSecondaryType {
    Compilation,
    Soundtrack,
    Spokenword,
    Interview,
    Audiobook,
    #[serde(rename = "Audio drama")]
    AudioDrama,
    Live,
    Remix,
    #[serde(rename = "DJ-mix")]
    DjMix,
    #[serde(rename = "Mixtape/Street")]
    MixtapeStreet,
    Demo,
    #[serde(rename = "Field recording")]
    FieldRecording,
}

impl From<MbSecondaryType> for SecondaryType {
    fn from(value: MbSecondaryType) -> Self {
        match value {
            MbSecondaryType::Compilation => Self::Compilation,
            MbSecondaryType::Soundtrack => Self::Soundtrack,
            MbSecondaryType::Spokenword => Self::Spokenword,
            MbSecondaryType::Interview => Self::Interview,
            MbSecondaryType::Audiobook => Self::Audiobook,
            MbSecondaryType::AudioDrama => Self::AudioDrama,
            MbSecondaryType::Live => Self::Live,
            MbSecondaryType::Remix => Self::Remix,
            MbSecondaryType::DjMix => Self::DjMix,
            MbSecondaryType::MixtapeStreet => Self::MixtapeStreet,
            MbSecondaryType::Demo => Self::Demo,
            MbSecondaryType::FieldRecording => Self::FieldRecording,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MbReleaseGroupResponse {
    #[serde(rename = "release-group-offset")]
    release_group_offset: i16,
    #[serde(rename = "release-group-count")]
    release_group_count: i16,
    #[serde(rename = "release-groups")]
    release_groups: Vec<MbReleaseGroupDto>,
}

#[async_trait]
impl ReleaseGroupFetcher for MbClient {
    async fn fetch_release_groups_by_artist(
        &self,
        mbid: Mbid<Artist>,
    ) -> Result<Vec<ReleaseGroupSummary>, anyhow::Error> {
        let response: MbReleaseGroupResponse = self
            .get_json(
                "release-group",
                &[
                    ("artist", mbid.get().to_string().as_str()),
                    ("type", "album|single|ep"),
                    ("limit", "100"),
                    ("offset", "0"),
                    ("fmt", "json"),
                ],
            )
            .await?;

        let release_groups: Vec<ReleaseGroupSummary> = response
            .release_groups
            .into_iter()
            .map(|dto| ReleaseGroupSummary {
                id: Mbid::new(dto.id),
                title: dto.title,
                primary_type: dto.primary_type.into(),
                release_date: dto.release_date,
                secondary_types: dto
                    .secondary_types
                    .into_iter()
                    .map(SecondaryType::from)
                    .collect(),
            })
            .collect();

        Ok(release_groups)
    }
}
