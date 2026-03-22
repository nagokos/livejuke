use crate::application::traits::password_hasher::PasswordHasher as AppPasswordHasher;
use anyhow::Ok;
use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub struct Argon2Hasher;

impl AppPasswordHasher for Argon2Hasher {
    fn hash(&self, password: &str) -> Result<String, anyhow::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate_password_digest: {}", e))?
            .to_string();
        Ok(password_hash)
    }
    fn verify(&self, password: &str, password_hash: &str) -> Result<bool, anyhow::Error> {
        let parsed_hash = PasswordHash::new(password_hash).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
