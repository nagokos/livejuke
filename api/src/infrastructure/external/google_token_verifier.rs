use std::sync::RwLock;

use async_trait::async_trait;
use jsonwebtoken::{DecodingKey, Validation, decode, decode_header, jwk::JwkSet};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::application::traits::{
    id_token_verifier::{IdTokenVerifier, OidcVerifyError},
    types::ExternalUserInfo,
};

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
    pub async fn new(client_id: String, http_client: Client) -> Result<Self, OidcVerifyError> {
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
    async fn refresh_keys(&self) -> Result<(), OidcVerifyError> {
        let response = self
            .http_client
            .get(GOOGLE_JWKS_URI)
            .send()
            .await?
            .text()
            .await?;
        let jwks: JwkSet = serde_json::from_str(&response)?;
        let mut write = self
            .cached_keys
            .write()
            .map_err(|_| OidcVerifyError::Internal("lock poisoned".to_string()))?;
        *write = jwks;

        Ok(())
    }
}

#[async_trait]
impl IdTokenVerifier for GoogleTokenVerifier {
    async fn verify(&self, id_token: &str) -> Result<ExternalUserInfo, OidcVerifyError> {
        let header = decode_header(id_token)
            .map_err(|e| OidcVerifyError::InvalidToken(format!("Header decode failed: {}", e)))?;

        let Some(kid) = header.kid else {
            return Err(OidcVerifyError::InvalidToken("Missing kid header".into()));
        };

        let jwk = {
            let jwks = self
                .cached_keys
                .read()
                .map_err(|_| OidcVerifyError::Internal("lock poisoned".to_string()))?;
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
                    .map_err(|_| OidcVerifyError::Internal("lock poisoned".to_string()))?;
                jwks.find(&kid).cloned().ok_or_else(|| {
                    OidcVerifyError::Internal("No matching JWK found for the given kid".to_string())
                })?
            }
        };

        let validation = {
            let mut validation = Validation::new(header.alg);
            validation.set_audience(&[&self.client_id]);
            validation.set_issuer(&["https://accounts.google.com"]);
            validation.validate_exp = true;
            validation
        };

        let decoded_token = decode::<GoogleResponse>(
            id_token,
            &DecodingKey::from_jwk(&jwk)
                .map_err(|e| OidcVerifyError::InvalidToken(e.to_string()))?,
            &validation,
        )
        .map_err(|e| {
            tracing::error!(error = %e, "failed to verify Google id_token");
            OidcVerifyError::InvalidToken(e.to_string())
        })?;

        if !decoded_token.claims.email_verified {
            return Err(OidcVerifyError::EmailNotVerified);
        }

        Ok(ExternalUserInfo {
            sub: decoded_token.claims.sub,
            email: decoded_token.claims.email,
        })
    }
}
