pub trait PasswordHasher {
    fn hash(&self, password: &str) -> Result<String, anyhow::Error>;
    fn verify(&self, password: &str) -> Result<bool, anyhow::Error>;
}
