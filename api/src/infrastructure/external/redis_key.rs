use redis::{ToRedisArgs, ToSingleRedisArg};

pub enum RedisKey<'a> {
    Verification(&'a str),
    RateLimitSendCode(&'a str),
    AttemptVerify(&'a str),
}

impl ToSingleRedisArg for RedisKey<'_> {}

impl ToRedisArgs for RedisKey<'_> {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let key = match self {
            Self::Verification(email) => format!("verification:{}", email),
            Self::RateLimitSendCode(email) => format!("rate:send-code:{}", email),
            Self::AttemptVerify(email) => format!("attempt:verify:{}", email),
        };
        key.write_redis_args(out);
    }
}
