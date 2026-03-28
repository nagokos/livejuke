use crate::{
    application::traits::types::{AccessToken, RefreshToken},
    domain::user::model::User,
};

#[derive(Debug)]
pub struct AuthResult {
    pub user: User,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
}
