use crate::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserGroup {
    pub username: String,
    pub group_name: String,
}
