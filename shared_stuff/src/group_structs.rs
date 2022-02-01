use crate::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GroupForm {
    pub username: String,
    pub group_name: String,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AddUser {
    pub username: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
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
