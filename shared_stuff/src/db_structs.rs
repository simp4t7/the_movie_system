use crate::groups_stuff::{GroupForm, GroupInfo};
use crate::{Deserialize, Serialize, YewMovieDisplay};
use std::collections::HashSet;
use uuid::Uuid;

//impl GroupData {
//pub fn new(input: GroupForm) -> Self {
//let mut members_hash = HashSet::new();
//members_hash.insert(input.username);
//let now = chrono::Utc::now().timestamp();
//let turn = String::from("");
//Self {
//group_name: input.group_name,
//members: members_hash,
//movies_watched: HashSet::new(),
//current_movies: HashSet::new(),
//turn,
//date_created: now,
//date_modified: now,
//}
//}
//}

//impl UserData {
//pub async fn new(input: UserInfo) -> Result<Self> {
//let id = Uuid::new_v4();
//let (hashed_password, salt) = hasher(&input.password).await?;
//let groups = HashSet::new();
//let now = chrono::Utc::now().timestamp();
//Ok(Self {
//id,
//hashed_password,
//salt,
//groups,
//date_created: now,
//date_modified: now,
//})
//}
//}

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
pub struct GroupData {
    pub group_name: String,
    pub members: HashSet<String>,
    pub movies_watched: HashSet<String>,
    pub current_movies: HashSet<YewMovieDisplay>,
    pub turn: String,
    pub date_created: i64,
    pub date_modified: i64,
}
