use crate::err_info;
use crate::error_handling::{Result, WarpRejections};
use crate::{ACCESS_EXP, REFRESH_EXP, TOKEN_SECRET};
use shared_stuff::{DoubleTokenResponse, SingleTokenResponse};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Filter, Rejection,
};

type WebResult<T> = std::result::Result<T, Rejection>;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use shared_stuff::{Claims, Token};
use warp::reject::custom;

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = decode::<Claims>(
                &jwt,
                &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
                &Validation::new(Algorithm::HS512),
            )
            .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;

            Ok(decoded.claims.username)
        }
        // Err(e) => return Err(custom(WarpRejections::AuthRejection(AuthError::AccessError))),
        Err(e) => Err(e),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    log::info!("header map: {:?}", &headers);
    match headers.get(AUTHORIZATION) {
        Some(v) => Ok(v.to_str().unwrap_or_default().to_string()),
        None => Err(custom(WarpRejections::AuthError(err_info!()))),
    }
    /*
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(custom(WarpRejections::AuthRejection(AuthError::InvalidAuthHeaderError))),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(custom(WarpRejections::AuthRejection(AuthError::NoAuthHeaderError)));
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
    warp::filters::header::header("authorization")
    Ok(auth_header.to_owned())
    */
}

// pub async fn auth(headers: HeaderMap<HeaderValue>) -> WebResult<String> {
pub async fn auth(token: String) -> Result<String> {
    let claims = verify_token(token)?;
    log::info!("auth claims: {:?}", claims);
    Ok(claims.username)
}

pub fn generate_access_token(username: String) -> Result<SingleTokenResponse> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let token_exp = now + *ACCESS_EXP;

    let token_claims = Claims {
        username,
        exp: token_exp,
        token: Token::Access,
    };
    log::info!("past claims inside");
    let access_token = encode(
        &Header::new(Algorithm::HS512),
        &token_claims,
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
    .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;

    Ok(SingleTokenResponse { access_token })
}

pub fn generate_double_token(username: String) -> Result<DoubleTokenResponse> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let token_exp = now + *ACCESS_EXP;

    let token_claims = Claims {
        username: username.clone(),
        exp: token_exp,
        token: Token::Access,
    };
    log::info!("past claims inside");
    let access_token = encode(
        &Header::new(Algorithm::HS512),
        &token_claims,
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
    .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;

    let refresh_exp = now + *REFRESH_EXP;
    let refresh_claims = Claims {
        username,
        exp: refresh_exp,
        token: Token::Refresh,
    };
    let refresh_token = encode(
        &Header::new(Algorithm::HS512),
        &refresh_claims,
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
    .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    let token_response = DoubleTokenResponse {
        access_token,
        refresh_token,
    };
    log::info!("{:?}", &token_response);
    Ok(token_response)
}

pub fn verify_token(token: String) -> Result<Claims> {
    let token = decode(
        &token,
        &DecodingKey::from_secret(TOKEN_SECRET.as_ref()),
        &Validation::new(Algorithm::HS512),
    )
    .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    let claims = token.claims;
    Ok(claims)
}

// argon2: pw hasher crate
pub async fn hasher(password: &str) -> Result<(String, String)> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?
        .to_string();
    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    assert!(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok());

    // It's pretty weird turning this SaltString to a String like this... Not too hard
    // to fix, but this is simplest for now...
    Ok((password_hash, salt.as_salt().to_string()))
}

// take the input password string, compute a hashed password with the salt (mapped with the
// username in the db), and then then compare the computed hashed with the db's hashed.
pub fn verify_pass(password: String, salt: String, hashed_pw: String) -> Result<bool> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    Ok(hashed_pw == password_hash.to_string())
}

// db: username | hash_password | the_salt
// verify: compute the input pw with the_salt of username, check the result against with the
// hash_password
