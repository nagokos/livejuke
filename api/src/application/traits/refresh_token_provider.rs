use crate::application::traits::types::RefreshToken;

pub trait RefreshTokenProvider {
    fn generate(&self) -> RefreshToken;
    fn hash(&self, token: &RefreshToken) -> String;
}
