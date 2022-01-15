pub mod add_movies_stuff;
pub mod groups_stuff;
pub mod omdb_structs;
pub mod utils;
pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;
use std::fmt;
use validator::Validate;
use validator::ValidationError;

//WHICH STUFF NEEDS TO BE SERIALIZE / DESERIALIZE? ¯\_(-_-)_/¯

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoubleTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SingleTokenResponse {
    pub access_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub code: u16,
    pub message: String,
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MovieDisplay {
    pub movie_id: String,
    pub movie_title: String,
    pub movie_year: Option<u32>,
    pub movie_images: Option<ImageData>,
    pub movie_stars: String,
    //pub media_type: MediaType,
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct YewMovieDisplay {
    pub movie_id: String,
    pub movie_title: String,
    pub movie_year: Option<u32>,
    pub movie_images: Option<ImageData>,
    pub movie_stars: String,
    pub added_by: String,
    //pub media_type: MediaType,
}

impl MovieDisplay {
    pub fn into_yew_display(self, added_by: String) -> YewMovieDisplay {
        YewMovieDisplay {
            movie_id: self.movie_id,
            movie_title: self.movie_title,
            movie_year: self.movie_year,
            movie_images: self.movie_images,
            movie_stars: self.movie_stars,
            added_by,
        }
    }
}

#[derive(Debug, Serialize, Validate, Deserialize, Clone)]
pub struct UserInfo {
    #[validate(email)]
    pub username: String,
    #[validate(custom = "zxcvbn_func")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImdbQuery {
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MovieDisplayResponse {
    pub movies: Vec<MovieDisplay>,
}

impl<T: ToString> From<T> for ImdbQuery {
    fn from(s: T) -> Self {
        Self {
            query: s.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginLookup {
    pub username: String,
    pub hashed_password: String,
    pub salt: String,
}

pub fn zxcvbn_func(password: &str) -> Result<(), ValidationError> {
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
    pub s: Option<String>,
    //pub link: Option<String>,
    pub id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JsonQuery {
    pub v: u32,
    pub q: String,
    pub d: Option<Vec<MovieInfo>>,
}

#[derive(Hash, Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ImageData {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
