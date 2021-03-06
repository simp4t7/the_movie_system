use crate::err_info;
use crate::error_handling::{Result, WarpRejections};
use crate::{ACCESS_EXP, REFRESH_EXP, TOKEN_SECRET};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use shared_stuff::auth_structs::{Claims, Token, TokenResponse};
use warp::reject::custom;
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Filter, Rejection,
};

//I'm pretty sure you can use the <Result> type from error_handling.
//If I'm wrong, you only used this one place so shouldn't be hard to revert it.
//type WebResult<T> = std::result::Result<T, Rejection>;

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .map(move |headers: HeaderMap<HeaderValue>| headers)
        .and_then(authorize)
}

async fn authorize(headers: HeaderMap<HeaderValue>) -> Result<String> {
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
        Err(e) => Err(e),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    log::info!("header map: {:?}", &headers);
    match headers.get(AUTHORIZATION) {
        Some(v) => Ok(v.to_str().unwrap_or_default().to_string()),
        None => Err(custom(WarpRejections::AuthError(err_info!()))),
    }
}

pub fn generate_tokens(username: String, token_type: Token) -> Result<TokenResponse> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let token_exp = now + *ACCESS_EXP;

    let token_claims = Claims {
        username: username.clone(),
        exp: token_exp,
        token: Token::Access,
    };
    let access_token = encode(
        &Header::new(Algorithm::HS512),
        &token_claims,
        &EncodingKey::from_secret(TOKEN_SECRET.as_bytes()),
    )
    .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    let mut token_resp = TokenResponse {
        access_token,
        refresh_token: None,
    };
    match token_type {
        Token::Access => Ok(token_resp),
        Token::Refresh => {
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
            token_resp.refresh_token = Some(refresh_token);
            Ok(token_resp)
        }
    }
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

pub fn verify_pass(password: String, salt: String, hashed_pw: String) -> Result<bool> {
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| custom(WarpRejections::AuthError(err_info!())))?;
    Ok(hashed_pw == password_hash.to_string())
}
