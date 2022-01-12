use sqlx::types::chrono::NaiveDateTime;

#[derive(Debug)]
pub struct GroupMembers {
    pub members: String,
}

#[derive(Debug)]
pub struct GroupName {
    pub name: String,
}

#[derive(Debug)]
pub struct UserGroups {
    pub groups: Option<String>,
}

#[derive(Debug)]
pub struct GroupCurrentMovies {
    pub current_movies: Option<String>,
}
#[derive(Debug)]
pub struct GroupId {
    pub id: String,
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub salt: String, // hash helper
    pub groups: Option<String>,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

#[derive(Debug)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub members: String,
}
