use rand::RngExt;
use sha2::{Digest, Sha256};

use crate::application::traits::{
    refresh_token_provider::RefreshTokenProvider, types::RefreshToken,
};

use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

pub struct OpaqueRefreshTokenProvider;

impl RefreshTokenProvider for OpaqueRefreshTokenProvider {
    fn generate(&self) -> RefreshToken {
        let mut bytes = [0u8; 32];
        rand::rng().fill(&mut bytes);
        RefreshToken::new(URL_SAFE_NO_PAD.encode(bytes))
    }
    fn hash(&self, token: &RefreshToken) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_str());
        hex::encode(hasher.finalize())
    }
}
