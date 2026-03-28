pub struct AuthConfig {
    pub refresh_token_expiration: i64,
}

impl AuthConfig {
    pub fn new(refresh_token_expiration: i64) -> Self {
        Self {
            refresh_token_expiration,
        }
    }
}
