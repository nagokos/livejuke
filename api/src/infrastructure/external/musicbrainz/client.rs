use std::time::Duration;

use anyhow::Context;
use reqwest::{Client, header};

pub fn build_musicbrainz_client(
    app_name: &str,
    app_version: &str,
    contact: &str,
) -> Result<Client, anyhow::Error> {
    let user_agent = format!("{}/{} ({})", app_name, app_version, contact);

    let mut default_headers = header::HeaderMap::new();
    default_headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );

    Client::builder()
        .connect_timeout(Duration::from_secs(5))
        .timeout(Duration::from_secs(15))
        .user_agent(user_agent)
        .default_headers(default_headers)
        .http2_prior_knowledge()
        .https_only(true)
        .build()
        .context("failed to build Mb HTTP client")
}
