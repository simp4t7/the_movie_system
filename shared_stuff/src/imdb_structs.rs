pub use serde::{Deserialize, Serialize};
pub use serde_json::Value;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImdbQuery {
    pub query: String,
}

impl<T: ToString> From<T> for ImdbQuery {
    fn from(s: T) -> Self {
        Self {
            query: s.to_string(),
        }
    }
}

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
    pub q: Option<String>,
    pub s: Option<String>,
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
