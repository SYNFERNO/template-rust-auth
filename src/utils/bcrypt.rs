use bcrypt::{hash_with_salt, verify, BcryptError, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, BcryptError> {
    hash_with_salt(password, DEFAULT_COST, [1u8; 16]).map(|hp| hp.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, BcryptError> {
    verify(password, hash)
}
