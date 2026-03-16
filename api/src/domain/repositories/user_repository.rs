use crate::domain::models::user::User;

pub trait UserRepository {
    fn create_user(&self) -> anyhow::Result<User>;
}
