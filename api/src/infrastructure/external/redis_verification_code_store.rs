use async_trait::async_trait;
use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::{
    application::traits::{
        types::VerificationData, verification_code_store::VerificationCodeStore,
    },
    domain::authentication::email::Email,
    infrastructure::external::redis_key::RedisKey,
};

pub struct RedisVerificationCodeStore {
    conn: MultiplexedConnection,
    max_attempts: i64,
    rate_limit: i64,
}

impl RedisVerificationCodeStore {
    pub fn new(conn: MultiplexedConnection, max_attempts: i64, rate_limit: i64) -> Self {
        Self {
            conn,
            max_attempts,
            rate_limit,
        }
    }
}

#[async_trait]
impl VerificationCodeStore for RedisVerificationCodeStore {
    fn is_rate_limited(&self, count: i64) -> bool {
        count > self.rate_limit
    }
    fn is_max_attempts(&self, count: i64) -> bool {
        count > self.max_attempts
    }
    async fn increment_rate_limit(&self, email: &Email) -> Result<i64, anyhow::Error> {
        let mut conn = self.conn.clone();
        let count: i64 = conn
            .incr(RedisKey::RateLimitSendCode(email.as_ref()), 1)
            .await?;
        if count == 1 {
            let _: () = conn
                .expire(RedisKey::RateLimitSendCode(email.as_ref()), 600)
                .await?;
        }

        Ok(count)
    }
    async fn increment_attempts(&self, email: &Email) -> Result<i64, anyhow::Error> {
        let mut conn = self.conn.clone();
        let count: i64 = conn
            .incr(RedisKey::AttemptVerify(email.as_ref()), 1)
            .await?;
        if count == 1 {
            let _: () = conn
                .expire(RedisKey::AttemptVerify(email.as_ref()), 300)
                .await?;
        }

        Ok(count)
    }
    async fn save(&self, email: &Email, data: &VerificationData) -> Result<(), anyhow::Error> {
        let mut conn = self.conn.clone();
        let json_data = serde_json::to_string(data)?;
        let _: () = conn
            .set_ex(RedisKey::Verification(email.as_ref()), json_data, 300)
            .await?;

        Ok(())
    }
    async fn find(&self, email: &Email) -> Result<Option<VerificationData>, anyhow::Error> {
        let mut conn = self.conn.clone();
        let res: Option<String> = conn.get(RedisKey::Verification(email.as_ref())).await?;
        res.map(|json| serde_json::from_str::<VerificationData>(&json).map_err(|e| e.into()))
            .transpose()
    }
    async fn delete(&self, email: &Email) -> Result<(), anyhow::Error> {
        let mut conn = self.conn.clone();
        let _: () = conn.del(RedisKey::Verification(email.as_ref())).await?;
        Ok(())
    }
}
