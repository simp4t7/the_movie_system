use crate::auth::verify_pass;
use crate::auth::verify_token;
use crate::auth::with_auth;
use crate::err_info;
use crate::error_handling::WarpRejections;
use crate::State;
use http::status::StatusCode;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::groups_stuff::{
    AddUser, BasicUsername, GroupForm, GroupInfo, GroupMoviesForm, UserGroupsJson,
};
use shared_stuff::{ErrorMessage, ImdbQuery, UserInfo};
use sqlx::SqlitePool;
use warp::reject::custom;

use crate::auth::generate_tokens;
use shared_stuff::Token;
use uuid::Uuid;

use warp::reply::json;
use warp::Filter;

use crate::new_db_stuff::{
    db_add_group_to_user, db_add_user_to_group, db_get_all_group_names,
    db_get_group_movies, db_get_user, db_get_user_groups, db_insert_group, db_insert_user,
    db_save_group_movies, db_user_leave_group, create_user_data, create_group_data, db_get_group,
};

/*
pub fn get_group_data(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_group_data")
        .and(warp::path::param())
        .and(with_db(state.db.clone()))
        .and_then(|group_id: String, db: SqlitePool| async move {
            match db_get_group(&db, &group_id) {
                Ok(group_data) => {
                    let json_resp = serde_json::to_string(&group_data)
                        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
                    Ok(json_resp)
                },
                Err(e) => Err(e),
            }

        })
}
*/
