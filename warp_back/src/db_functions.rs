use crate::auth::verify_pass;
use std::collections::HashSet;

use crate::error_handling::{AuthError, Result, SqlxError, WarpRejections};

use shared_stuff::groups_stuff::AddUser;
use shared_stuff::MovieDisplay;

use shared_stuff::groups_stuff::GroupForm;
use shared_stuff::groups_stuff::GroupMoviesForm;
use shared_stuff::LoginLookup;
use shared_stuff::UserInfo;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;
use sqlx::{query, query_as};
use uuid::Uuid;
use warp::reject::custom;

use crate::auth::hasher;

// NOTHING WRONG WITH 200 LINE FUNCTIONS...
