use crate::MovieDisplay;
use crate::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupForm {
    pub username: String,
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNames {
    pub groups: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupsId {
    pub groups: HashSet<String>,
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
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AddUser {
    pub username: String,
    pub add_user: String,
}
