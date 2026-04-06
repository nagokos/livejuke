use crate::application::traits::types::RefreshToken;

pub trait RefreshTokenProvider: Send + Sync {
    fn generate(&self) -> RefreshToken;
    fn hash(&self, token: &RefreshToken) -> String;
}
