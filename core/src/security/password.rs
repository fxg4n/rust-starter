use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Algorithm, Version, Params,
};
use crate::core::constants::security::{
    PASSWORD_HASH_MEMORY_COST,
    PASSWORD_HASH_TIME_COST,
    PASSWORD_HASH_PARALLELISM,
};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(
        PASSWORD_HASH_MEMORY_COST,
        PASSWORD_HASH_TIME_COST,
        PASSWORD_HASH_PARALLELISM,
        None,
    ).unwrap();
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
