use crate::application::traits::password_hasher::PasswordHasher as AppPasswordHasher;
use anyhow::Ok;
use argon2::{
    Argon2, PasswordHasher,
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
    fn verify(&self, password: &str) -> Result<bool, anyhow::Error> {
        todo!()
    }
}
