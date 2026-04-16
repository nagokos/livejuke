use crate::{
    application::traits::types::{AccessToken, RefreshToken},
    domain::user::model::UserAuthDetail,
};

#[derive(Debug)]
pub struct AuthResult {
    pub user_auth_detail: UserAuthDetail,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
