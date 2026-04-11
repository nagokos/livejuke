pub struct AuthConfig {
    pub refresh_token_exp_secs: u64,
}

impl AuthConfig {
    pub fn new(refresh_token_exp_secs: u64) -> Self {
        Self {
            refresh_token_exp_secs,
        }
    }
}
