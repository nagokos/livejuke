use std::sync::RwLock;

use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::application::traits::{id_token_verifier::IdTokenVerifier, types::ExternalUserInfo};

const GOOGLE_JWKS_URI: &str = "https://www.googleapis.com/oauth2/v3/certs";

#[derive(Deserialize, Serialize, Debug)]
struct GoogleResponse {
    iss: String,
    aud: String,
    sub: String,
    email: String,
    email_verified: bool,
}

pub struct GoogleTokenVerifier {
    client_id: String,
    http_client: Client,
    cached_keys: RwLock<JwkSet>,
}

impl GoogleTokenVerifier {
    pub async fn new(client_id: String, http_client: Client) -> Result<Self, anyhow::Error> {
        let response = http_client
            .get(GOOGLE_JWKS_URI)
            .send()
            .await?
            .text()
            .await?;
        let response: JwkSet = serde_json::from_str(&response)?;

        let cached_keys = RwLock::new(response);

        Ok(Self {
            client_id,
            http_client,
            cached_keys,
        })
    }
    async fn refresh_keys(&self) -> Result<(), anyhow::Error> {
        let response = self
            .http_client
            .get("https://www.googleapis.com/oauth2/v3/certs")
            .send()
            .await?
            .text()
            .await?;
        let jwks: JwkSet = serde_json::from_str(&response)?;
        let mut write = self
            .cached_keys
            .write()
            .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
        *write = jwks;

        Ok(())
    }
}

#[async_trait]
impl IdTokenVerifier for GoogleTokenVerifier {
    async fn verify(&self, id_token: &str) -> Result<ExternalUserInfo, anyhow::Error> {
        let header = decode_header(id_token).map_err(|e| {
            tracing::error!(error = %e, "failed to decode JWT header");
            e
        })?;

        let Some(kid) = header.kid else {
            return Err(anyhow::anyhow!("Token doesn't have a `kid` header field"));
        };

        let jwk = {
            let jwks = self
                .cached_keys
                .read()
                .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
            jwks.find(&kid).cloned()
        };

        let jwk = match jwk {
            Some(jwk) => jwk,
            None => {
                tracing::warn!(kid = %kid, "JWK not found in cache, refreshing");
                self.refresh_keys().await.map_err(|e| {
                    tracing::error!(error = %e, "failed to refresh JWKS");
                    e
                })?;
                let jwks = self
                    .cached_keys
                    .read()
                    .map_err(|_| anyhow::anyhow!("lock poisoned"))?;
                jwks.find(&kid)
                    .cloned()
                    .ok_or_else(|| anyhow::anyhow!("No matching JWK found for the given kid"))?
            }
        };

        let validation = {
            let mut validation = Validation::new(header.alg);
            validation.set_audience(&[&self.client_id]);
            validation.set_issuer(&["https://accounts.google.com"]);
            validation.validate_exp = true;
            validation
        };

        let decoded_token =
            decode::<GoogleResponse>(id_token, &DecodingKey::from_jwk(&jwk)?, &validation)
                .map_err(|e| {
                    tracing::error!(error = %e, "failed to verify Google id_token");
                    e
                })?;

        if !decoded_token.claims.email_verified {
            tracing::error!(email = %decoded_token.claims.email, "email not verified");
            return Err(anyhow::anyhow!(
                "Google account email address has not been verified"
            ));
        }

        Ok(ExternalUserInfo {
            sub: decoded_token.claims.sub,
            email: decoded_token.claims.email,
        })
    }
}
