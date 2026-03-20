use crate::domain::{
    id::Id,
    user::model::{Role, User},
};

pub trait TokenProvider {
    fn generate(&self, sub: Id<User>, role: Role) -> Result<String, anyhow::Error>;
}
