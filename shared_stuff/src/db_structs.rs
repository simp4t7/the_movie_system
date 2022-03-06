use crate::group_structs::{GroupInfo, GroupUser};
use crate::shared_structs::{SystemState, YewMovieDisplay};
use crate::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBUser {
    pub username: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBUserStruct {
    pub username: String,
    pub user_data: UserData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: String,
    pub hashed_password: String,
    pub salt: String,
    pub groups: HashSet<GroupInfo>,
    pub date_created: i64,
    pub date_modified: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBGroup {
    pub id: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBGroupStruct {
    pub id: String,
    pub group_data: GroupData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupData {
    pub group_name: String,
    pub members: GroupUser,
    pub system_order: VecDeque<String>,
    pub movies_watched: HashSet<String>,
    pub current_movies: HashSet<YewMovieDisplay>,
    pub system_state: SystemState,
    pub turn: String,
    pub date_created: i64,
    pub date_modified: i64,
}

impl GroupData {
    pub fn new_empty() -> GroupData {
        GroupData {
            group_name: String::from(""),
            members: HashMap::new(),
            system_order: VecDeque::new(),
            movies_watched: HashSet::new(),
            current_movies: HashSet::new(),
            system_state: SystemState::AddingMovies,
            turn: String::from(""),
            date_created: 0,
            date_modified: 0,
        }
    }
    pub fn into_db_group_struct(self, id: &str) -> DBGroupStruct {
        DBGroupStruct {
            id: id.to_string(),
            group_data: self,
        }
    }
}
