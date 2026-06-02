use chrono::NaiveDate;

use crate::domain::{
    mbid::Mbid,
    release_group::model::{PrimaryType, ReleaseGroup, SecondaryType},
};

#[derive(Debug)]
pub struct ReleaseGroupSummary {
    pub id: Mbid<ReleaseGroup>,
    pub title: String,
    pub primary_type: PrimaryType,
    pub release_date: Option<NaiveDate>,
    pub secondary_types: Vec<SecondaryType>,
}
