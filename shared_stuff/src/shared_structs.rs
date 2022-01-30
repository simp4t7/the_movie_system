use crate::imdb_structs::ImageData;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SystemState {
    AddingMovies,
    SystemStarted,
    Finished,
}

#[derive(Hash, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct MovieDisplay {
    pub movie_id: String,
    pub movie_title: String,
    pub movie_year: u32,
    pub movie_images: ImageData,
    pub movie_stars: String,
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

#[derive(Hash, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct YewMovieDisplay {
    pub movie_id: String,
    pub movie_title: String,
    pub movie_year: u32,
    pub movie_images: ImageData,
    pub movie_stars: String,
    pub added_by: String,
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
