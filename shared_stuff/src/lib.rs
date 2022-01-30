pub mod auth_structs;
pub mod db_structs;
pub mod group_structs;
pub mod imdb_structs;
pub mod omdb_structs;
pub mod shared_structs;
pub mod utils;

pub use serde::{Deserialize, Serialize};

//WHICH STUFF NEEDS TO BE SERIALIZE / DESERIALIZE? ¯\_(-_-)_/¯

//#[derive(Debug, Serialize, Deserialize)]
//pub struct LoginLookup {
//pub username: String,
//pub hashed_password: String,
//pub salt: String,
//}

//MAYBE ADD THIS FOR TV SERIES / MOVIE / OTHER TYPES
