use crate::MovieDisplay;
use crate::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupsId {
    pub groups: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupStruct {
    pub id: String,
    pub username: String,
    pub movies_watched: Vec<WatchedMovies>,
    pub current_movies: Vec<MovieDisplay>,
    pub turn: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WatchedMovies {
    pub movie: MovieDisplay,
    pub rating: UserRating,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserRating {
    pub user: BasicUsername,
    pub rating: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CurrentMovies {
    pub movie: MovieDisplay,
    pub added_by: BasicUsername,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BasicUsername {
    pub username: String,
}
