use crate::groups_stuff::GroupInfo;
use crate::{Deserialize, Serialize, YewMovieDisplay};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBUser {
    pub username: String,
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserData {
    pub id: Uuid,
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
pub enum SystemState {
    AddingMovies,
    SystemStarted,
    Finished,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct GroupData {
    pub group_name: String,
    pub members: VecDeque<String>,
    pub movies_watched: HashSet<String>,
    pub current_movies: HashSet<YewMovieDisplay>,
    pub ready_status: HashMap<String, bool>,
    pub system_state: SystemState,
    pub turn: String,
    pub date_created: i64,
    pub date_modified: i64,
}

impl GroupData {
    pub fn new_empty() -> GroupData {
        GroupData {
            group_name: String::from(""),
            members: VecDeque::new(),
            movies_watched: HashSet::new(),
            current_movies: HashSet::new(),
            ready_status: HashMap::new(),
            system_state: SystemState::AddingMovies,
            turn: String::from(""),
            date_created: 0,
            date_modified: 0,
        }
    }
}
