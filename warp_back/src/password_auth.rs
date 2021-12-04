use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
// argon2: pw hasher crate

// TAKE CARE OF THE unwraps()
pub async fn hasher(password: &str) -> Result<(String, String)> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();
    assert!(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());
    Ok((password_hash, salt.as_str().to_string()))
}

// take the input password string, compute a hashed password with the salt (mapped with the
// username in the db), and then then compare the computed hashed with the db's hashed.
pub fn verify_pass(password: String, salt: String, hashed_pw: String) -> bool {
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
    hashed_pw == password_hash.to_string()
}

// db: username | hash_password | the_salt
// verify: compute the input pw with the_salt of username, check the result against with the
// hash_password
