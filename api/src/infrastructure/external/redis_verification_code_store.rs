use std::time::Duration;

use async_trait::async_trait;
use redis::{AsyncCommands, Script, ToSingleRedisArg, aio::MultiplexedConnection};

use crate::{
    application::traits::{
        types::VerificationData, verification_code_store::VerificationCodeStore,
    },
    domain::authentication::email::Email,
    infrastructure::external::redis_key::RedisKey,
};

pub struct RedisVerificationCodeStore {
    conn: MultiplexedConnection,
    verification_code_exp_secs: u64,
    max_attempts: i64,
    max_attempts_ttl_secs: u64,
    rate_limit: i64,
    rate_limit_ttl_secs: u64,
}

impl RedisVerificationCodeStore {
    pub fn new(
        conn: MultiplexedConnection,
        verification_code_exp_secs: u64,
        max_attempts: i64,
        max_attempts_ttl_secs: u64,
        rate_limit: i64,
        rate_limit_ttl_secs: u64,
    ) -> Self {
        Self {
            conn,
            verification_code_exp_secs,
            max_attempts,
            max_attempts_ttl_secs,
            rate_limit,
            rate_limit_ttl_secs,
        }
    }
    async fn atomic_incr_expire<K>(&self, key: K, ttl: Duration) -> Result<i64, anyhow::Error>
    where
        K: ToSingleRedisArg + Sync + Send,
    {
        let mut conn = self.conn.clone();

        let script = Script::new(
            r#"
            local count = redis.call("INCR", KEYS[1])
            if count == 1 then
                redis.call("EXPIRE", KEYS[1], ARGV[1])
            end 
            return count
            "#,
        );
        let ttl_secs = ttl.as_secs() as i64;
        let count: i64 = script
            .key(key)
            .arg(ttl_secs)
            .invoke_async(&mut conn)
            .await?;

        Ok(count)
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
        let count = self
            .atomic_incr_expire(
                RedisKey::RateLimitSendCode(email.as_ref()),
                Duration::from_secs(self.rate_limit_ttl_secs),
            )
            .await?;

        Ok(count)
    }
    async fn increment_attempts(&self, email: &Email) -> Result<i64, anyhow::Error> {
        let count = self
            .atomic_incr_expire(
                RedisKey::AttemptVerify(email.as_ref()),
                Duration::from_secs(self.max_attempts_ttl_secs),
            )
            .await?;

        Ok(count)
    }
    async fn save(&self, email: &Email, data: &VerificationData) -> Result<(), anyhow::Error> {
        let mut conn = self.conn.clone();
        let json_data = serde_json::to_string(data)?;
        let _: () = conn
            .set_ex(
                RedisKey::Verification(email.as_ref()),
                json_data,
                self.verification_code_exp_secs,
            )
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
