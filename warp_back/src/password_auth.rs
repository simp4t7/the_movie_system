use crate::error_handling::AuthError;
use crate::error_handling::Result;
use crate::error_handling::WarpRejections;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use warp::reject::custom;
// argon2: pw hasher crate

pub async fn hasher(password: &str) -> Result<(String, String)> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| custom(WarpRejections::AuthRejection(AuthError::HasherError)))?
        .to_string();
    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|_| custom(WarpRejections::AuthRejection(AuthError::HasherError)))?;
    assert!(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
    Ok((password_hash, salt.as_str().to_string()))
}

// take the input password string, compute a hashed password with the salt (mapped with the
// username in the db), and then then compare the computed hashed with the db's hashed.
pub fn verify_pass(password: String, salt: String, hashed_pw: String) -> Result<bool> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| custom(WarpRejections::AuthRejection(AuthError::VerifyError)))?;
    Ok(hashed_pw == password_hash.to_string())
}

// db: username | hash_password | the_salt
// verify: compute the input pw with the_salt of username, check the result against with the
// hash_password
