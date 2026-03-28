use crate::{
    application::traits::types::{AccessToken, CurrentUser},
    domain::{
        id::Id,
        user::model::{Role, User},
    },
};

pub trait AccessTokenProvider {
    fn generate(&self, sub: Id<User>, role: Role) -> Result<AccessToken, anyhow::Error>;
    fn verify(&self, token: &str) -> Result<CurrentUser, anyhow::Error>;
}
