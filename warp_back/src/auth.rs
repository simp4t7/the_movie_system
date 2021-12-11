use crate::error_handling::AuthError;
use crate::error_handling::Result;
use crate::error_handling::WarpRejections;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use lazy_static::lazy_static;
use shared_stuff::Claims;
use std::fs::File;
use std::io::Read;
use warp::reject::custom;

lazy_static! {
    static ref JWT_SECRET: String = {
        let mut current_dir = std::env::current_dir().expect("problem with current_dir");
        current_dir.push("test_keys");
        current_dir.push("jwt_secret.txt");
        println!("{:?}", &std::env::current_dir());
        println!("{:?}", &current_dir);
        let mut file = File::open(current_dir).expect("problem opening file");
        let mut secret = String::new();
        file.read_to_string(&mut secret)
            .expect("problem reading file");
        secret
    };
}

pub fn generate_jwt(username: String) -> Result<String> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let exp = now + 60;

    let my_claims = Claims { username, exp };
    log::info!("past claims inside");
    let token = encode(
        &Header::new(Algorithm::HS512),
        &my_claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| custom(WarpRejections::AuthRejection(AuthError::TokenError)))?;
    log::info!("{:?}", &token);
    Ok(token)
}

pub fn decode_token(token: String, secret: String) -> Result<TokenData<Claims>> {
    let token = decode(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| custom(WarpRejections::AuthRejection(AuthError::TokenError)))?;
    Ok(token)
}

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
