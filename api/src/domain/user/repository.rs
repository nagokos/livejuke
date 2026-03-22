use crate::domain::{id::Id, user::model::User};

pub trait UserRepository {
    fn find_by_id(
        &self,
        user_id: Id<User>,
    ) -> impl Future<Output = Result<Option<User>, anyhow::Error>> + Send;
}
