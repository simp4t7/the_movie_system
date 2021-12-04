pub mod utils;
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;
use std::fmt;
use validator::Validate;
use validator::ValidationError;
use zxcvbn::zxcvbn;

//WHICH STUFF NEEDS TO BE SERIALIZE / DESERIALIZE? ¯\_(-_-)_/¯

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDisplay {
    pub movie_title: String,
    pub movie_year: Option<u32>,
    pub movie_images: Option<ImageData>,
    //pub media_type: MediaType,
}
#[derive(Debug, Serialize, Validate, Deserialize, Clone)]
pub struct UserInfo {
    #[validate(email)]
    pub username: String,
    #[validate(custom = "zxcvbn_func")]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginLookup {
    pub username: String,
    pub hashed_password: String,
    pub salt: String,
}

pub fn zxcvbn_func(password: &String) -> Result<(), ValidationError> {
    if password == "password123" {
        return Err(ValidationError::new("umm, bad password"));
    }
    Ok(())
}

impl fmt::Display for MovieDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}, {:?}, {:?}",
            self.movie_title, self.movie_year, self.movie_images
        )
    }
}

//MAYBE ADD THIS FOR TV SERIES / MOVIE / OTHER TYPES
#[derive(Debug)]
pub enum MediaType {
    Movie,
    MiniSeries,
    TV,
    Other,
    None,
}

impl MediaType {
    pub fn new(input: Option<String>) -> Self {
        if let Some(media) = input {
            match media.as_str() {
                "feature" => Self::Movie,
                "TV mini series" => Self::MiniSeries,
                "TV series" => Self::TV,
                _ => Self::Other,
            }
        } else {
            Self::None
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct MovieInfo {
    pub l: Option<String>,
    pub i: Option<Value>,
    pub y: Option<u32>,
    pub q: Option<String>, // MediaType: feature / tv series
    //pub s: Option<String>,
    //pub link: Option<String>,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonQuery {
    pub v: u32,
    pub q: String,
    pub d: Option<Vec<MovieInfo>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ImageData {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
