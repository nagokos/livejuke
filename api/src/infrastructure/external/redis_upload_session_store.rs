use async_trait::async_trait;
use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::{
    application::traits::upload_session_store::UploadSessionStore,
    domain::{id::Id, user::model::User},
    infrastructure::external::redis_key::RedisKey,
};

const UPLOAD_SESSION_EXPIRES_SECS: u64 = 300;

pub struct RedisUploadSessionStore {
    conn: MultiplexedConnection,
}

impl RedisUploadSessionStore {
    pub fn new(conn: MultiplexedConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UploadSessionStore for RedisUploadSessionStore {
    async fn set_pending_upload<'a>(
        &'a self,
        key: Id<User>,
        value: &'a str,
    ) -> Result<(), anyhow::Error> {
        let mut conn = self.conn.clone();
        let _: () = conn
            .set_ex(
                RedisKey::UploadSession(&key.get().to_string()),
                value,
                UPLOAD_SESSION_EXPIRES_SECS,
            )
            .await?;
        Ok(())
    }
    async fn get_pending_upload(&self, key: Id<User>) -> Result<String, anyhow::Error> {
        let mut conn = self.conn.clone();
        let Some(avatar_key) = conn
            .get_del(RedisKey::UploadSession(&key.get().to_string()))
            .await?
        else {
            return Err(anyhow::anyhow!("No pending avatar upload found"));
        };
        Ok(avatar_key)
    }
}
