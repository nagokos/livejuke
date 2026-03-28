use crate::application::traits::types::ExternalUserInfo;

pub trait IdTokenVerifier {
    fn verify(
        &self,
        id_token: &str,
    ) -> impl Future<Output = Result<ExternalUserInfo, anyhow::Error>> + Send;
}
