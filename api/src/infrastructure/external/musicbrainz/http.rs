use std::{sync::Arc, time::Duration};

use reqwest::{Client, StatusCode};
use serde::de::DeserializeOwned;

use crate::infrastructure::external::musicbrainz::rate_limiter::MbRateLimiter;

pub struct MbClient {
    http_client: Client,
    base_url: String,
    rate_limiter: Arc<MbRateLimiter>,
}

impl MbClient {
    pub fn new(http_client: Client, rate_limiter: Arc<MbRateLimiter>) -> Self {
        Self {
            http_client,
            base_url: "https://musicbrainz.org/ws/2".to_string(),
            rate_limiter,
        }
    }
    pub(crate) async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T, anyhow::Error> {
        tokio::time::timeout(Duration::from_secs(10), self.rate_limiter.acquire()).await?;

        let url = format!("{}/{}", self.base_url, path);
        let response = self.http_client.get(url).query(query).send().await?;

        match response.status() {
            StatusCode::OK => Ok(response.json().await?),
            _ => Err(anyhow::anyhow!(
                "reqwest response error: {}",
                response.status()
            )),
        }
    }
}
