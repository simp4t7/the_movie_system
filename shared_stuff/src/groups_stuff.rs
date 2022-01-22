use crate::{Deserialize, Serialize};
use crate::{MovieDisplay, YewMovieDisplay};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupForm {
    pub username: String,
    pub group_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupMoviesForm {
    pub username: String,
    pub group_name: String,
    pub current_movies: HashSet<YewMovieDisplay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupNames {
    pub groups: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroupsJson {
    pub groups: HashSet<GroupInfo>,
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
    pub new_member: String,
    pub group_name: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq,)]
pub struct GroupInfo {
    pub name: String,
    pub uuid: String,
}

impl fmt::Display for GroupInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(GroupInfo: uuid: {}, name: {})", self.uuid, self.name)
    }
}
