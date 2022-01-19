use crate::auth::verify_pass;
use crate::auth::verify_token;
use crate::auth::with_auth;
use crate::error_handling::{AuthError, WarpRejections};
use crate::State;
use http::status::StatusCode;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::groups_stuff::{
    AddUser, BasicUsername, GroupForm, GroupMoviesForm, UserGroupsJson,
};
use shared_stuff::{ErrorMessage, ImdbQuery, UserInfo};
use sqlx::SqlitePool;
use warp::reject::custom;

use crate::auth::{generate_access_token, generate_double_token};
use crate::error_handling::SqlxError;
use uuid::Uuid;

use warp::reply::json;
use warp::Filter;

// domain/search -> do seach()
// why bytes? some of these could def use json since requests come in as json

// getthe search string -> bytes
// use the bytes -> &str -> send to the autocomplete_func and get back the results
// results ok -> turn it to json and send it back to yew
// results bad -> 400 bad request. //TODO currently it's 404

//pub fn test_route() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
//warp::path("test").map(|| "Hello, World!")
//}

use crate::new_db_stuff::{
    db_add_group_to_user, db_add_user_to_group, db_get_all_group_names, db_get_group_movies,
    db_get_user, db_get_user_groups, db_insert_group, db_insert_user, db_save_group_movies,
    db_user_leave_group, GroupData, UserData,
};

pub fn get_group_movies(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_group_movies")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|group_form: GroupForm, db: SqlitePool| async move {
            match db_get_group_movies(&db, &group_form).await {
                Ok(movies) => {
                    //let resp = MovieDisplayResponse { movies };
                    let json_resp = serde_json::to_string(&movies)
                        .map_err(|_| custom(WarpRejections::SerializationError))?;
                    Ok(json_resp)
                }
                Err(e) => Err(e),
            }
        })
}

pub fn save_group_movies(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("save_group_movies")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|group_movies: GroupMoviesForm, db: SqlitePool| async move {
            match db_save_group_movies(&db, &group_movies).await {
                Ok(_) => Ok(warp::reply()),
                Err(e) => {
                    log::info!("error is: {:?}", &e);
                    Err(e)
                }
            }
        })
}

pub fn add_user_to_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add_user")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|user: AddUser, db: SqlitePool| async move {
            match db_add_user_to_group(&db, &user).await {
                Ok(_) => Ok(warp::reply()),
                Err(e) => Err(e),
            }
        })
}

pub fn get_groups(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    log::info!("Entering get_groups");
    warp::path("get_groups")
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |user: BasicUsername, username: String, db: SqlitePool| async move {
                log::info!("username: {:?}", &user);
                match db_get_user_groups(&db, &user.username).await {
                    Ok(groups) => {
                        log::info!("okay, got the groups: {:?}", &groups);
                        let group_names = db_get_all_group_names(&db, &user.username).await?;
                        let groups_struct = UserGroupsJson {
                            groups: group_names,
                        };
                        let json_resp = serde_json::to_string(&groups_struct)
                            .map_err(|_| custom(WarpRejections::SerializationError))?;
                        log::info!("get group json_resp: {:?}", &json_resp);
                        Ok(json_resp)
                    }
                    Err(_e) => Err(custom(WarpRejections::SqlxRejection(
                        SqlxError::AddUserError,
                    ))),
                }
            },
        )
}

//pub fn get_groups1(
//state: &State,
//) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//log::info!("Entering get_groups1");
//warp::path("get_groups")
//.and(warp::body::json())
//.and(with_auth())
//.and(with_db(state.db.clone()))
//.and_then(
//|user: BasicUsername, username: String, db: SqlitePool| async move {
//log::info!(
//"Passed all wrappers. BasicUsername: {:?}, username: {:?}",
//&user,
//&username
//);
//match get_user_groups(&db, &username).await {
//Ok(groups) => {
//let new_vec = string_to_vec(groups);
//let group_names = get_all_group_names(&db, new_vec.clone()).await?;
//log::info!("get group group_names: {:?}", &group_names);
//let groups_struct = UserGroupsJson {
//groups: group_names,
//};
//let json_resp = serde_json::to_string(&groups_struct)
//.map_err(|_| custom(WarpRejections::SerializationError))?;
//log::info!("get group json_resp: {:?}", &json_resp);
//Ok(json_resp)
//}
//Err(e) => Err(e),
//}
//},
//)
//}

pub fn leave_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("leave_group")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|group_form: GroupForm, db: SqlitePool| async move {
            match db_user_leave_group(&db, &group_form).await {
                Ok(_) => Ok(warp::reply()),
                Err(e) => Err(e),
            }
        })
}

pub fn create_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("create_group")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|group_form: GroupForm, db: SqlitePool| async move {
            let uuid = Uuid::new_v4();
            let uuid_string = uuid.to_string();
            let group_data = GroupData::new(group_form.clone());
            match db_insert_group(&db, &uuid_string, group_data).await {
                Ok(_) => {
                    let user_data = db_get_user(&db, &group_form.username).await?;
                    db_add_group_to_user(&db, user_data, (group_form.group_name, uuid)).await?;
                    Ok(warp::reply())
                }
                Err(e) => Err(e),
            }
        })
}
pub fn authorize_refresh(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("refresh_auth")
        .and(warp::filters::header::header("authorization"))
        .map(|token: String| match verify_token(token) {
            Ok(claims) => {
                let username = claims.username;
                if let Ok(token_response) = generate_access_token(username) {
                    let code = StatusCode::OK;
                    let reply = warp::reply::json(&token_response);
                    warp::reply::with_status(reply, code)
                } else {
                    let code = StatusCode::BAD_REQUEST;
                    let reply = warp::reply::json(&ErrorMessage {
                        code: code.into(),
                        message: WarpRejections::AuthRejection(AuthError::AccessError).into(),
                    });
                    warp::reply::with_status(reply, code)
                }
            }
            Err(_) => {
                let code = StatusCode::UNAUTHORIZED;
                let reply = warp::reply::json(&ErrorMessage {
                    code: code.into(),
                    message: WarpRejections::AuthRejection(AuthError::AccessError).into(),
                });
                warp::reply::with_status(reply, code)
            }
        })
        .with(&state.cors)
}

pub fn authorize_access(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("access_auth")
        .and(warp::filters::header::header("authorization"))
        .map(|token: String| match verify_token(token) {
            Ok(claims) => {
                let code = StatusCode::OK;
                let reply = warp::reply::json(&claims);
                warp::reply::with_status(reply, code)
            }
            Err(_) => {
                let code = StatusCode::UNAUTHORIZED;
                let reply = warp::reply::json(&ErrorMessage {
                    code: code.into(),
                    message: WarpRejections::AuthRejection(AuthError::AccessError).into(),
                });
                warp::reply::with_status(reply, code)
            }
        })
        .with(&state.cors)
}

pub fn search(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("search")
        .and(warp::body::json())
        .and_then(|query: ImdbQuery| async move {
            match autocomplete_func(query).await {
                Ok(movie_vec) => {
                    log::info!("{:?}", &movie_vec);
                    let json_res = json(&movie_vec);
                    Ok(json_res)
                }
                Err(e) => Err(custom(WarpRejections::AutocompleteError)),
            }
        })
        .with(&state.cors)
}

pub fn register(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|user_info: UserInfo, db: SqlitePool| async move {
            let user_data = UserData::new(user_info.clone()).await?;
            match db_insert_user(&db, &user_info.username, user_data).await {
                Ok(_e) => Ok(warp::reply()),
                Err(e) => Err(e),
            }
        })
        .with(&state.cors)
}

pub fn login(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|user: UserInfo, db: SqlitePool| async move {
            let user_info = db_get_user(&db, &user.username).await?;
            let token_response = generate_double_token(user_info.0.clone())?;
            log::info!("user_info: {:?}", &user_info);
            match verify_pass(user.password, user_info.1.salt, user_info.1.hashed_password)? {
                true => Ok(json(&token_response)),
                false => Err(custom(WarpRejections::AuthRejection(
                    AuthError::VerifyError,
                ))),
            }
        })
        .with(&state.cors)
    //.with(warp::reply::with::header("Authorization", token))
}

fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
