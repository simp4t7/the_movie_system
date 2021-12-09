use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    username: String,
    company: String,
    exp: usize,
}

pub fn token_stuff() -> Result<String, Box<dyn std::error::Error>> {
    let my_claims = Claims {
        username: "test".into(),
        company: "I dont know".into(),
        exp: 98123,
    };
    let token = encode(
        &Header::new(Algorithm::HS512),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();
    Ok(token)
}
