use crate::auth::{verify_pass, verify_token, with_auth};
use crate::err_info;
use crate::error_handling::WarpRejections;
use crate::State;
use http::status::StatusCode;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::db_structs::DBGroupStruct;
use shared_stuff::groups_stuff::{AddUser, BasicUsername, GroupForm, GroupInfo, UserGroupsJson};
use shared_stuff::{ErrorMessage, ImdbQuery, UserInfo};
use sqlx::SqlitePool;
use warp::reject::custom;

use crate::auth::generate_tokens;
use shared_stuff::Token;
use uuid::Uuid;

use warp::reply::json;
use warp::Filter;

use crate::new_db_stuff::{
    create_group_data, create_user_data, db_add_group_to_user, db_add_user_to_group,
    db_add_user_to_group1, db_get_all_group_names, db_get_group_movies, db_get_user,
    db_get_user_groups, db_insert_group, db_insert_user, db_save_group_movies,
    db_user_leave_group1, verify_group_member,
};

pub fn get_group_data(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_group_data")
        .and(warp::path::param())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_id: String, username: String, db: SqlitePool| async move {
                log::info!("group_id: {:?}", &group_id);
                match verify_group_member(group_id, username, &db).await {
                    Ok(group_data) => {
                        let json_resp = serde_json::to_string(&group_data)
                            .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
                        Ok(json_resp)
                    }
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn add_user_to_group_param(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add_user")
        .and(warp::path::param())
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_id: String, add_user: BasicUsername, _username: String, db: SqlitePool| async move {
                let add_username = add_user.username;
                match db_add_user_to_group1(&group_id, add_username, &db).await {
                    Ok(_) => Ok(warp::reply()),
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn leave_group1(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("leave_group")
        .and(warp::path::param())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_id: String, username: String, db: SqlitePool| async move {
                match db_user_leave_group1(&db, username, &group_id).await {
                    Ok(_) => Ok(warp::reply()),
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn add_user_to_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add_user")
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |user: AddUser, _username: String, db: SqlitePool| async move {
                match db_add_user_to_group(&db, &user).await {
                    Ok(_) => Ok(warp::reply()),
                    Err(e) => Err(e),
                }
            },
        )
}

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
                        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
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
        .and_then(|group_struct: DBGroupStruct, db: SqlitePool| async move {
            log::info!("save_group_movies -> group_struct: {:?}", &group_struct);
            match db_save_group_movies(&db, &group_struct).await {
                Ok(_) => Ok(warp::reply()),
                Err(e) => {
                    log::info!("error is: {:?}", &e);
                    Err(e)
                }
            }
        })
}

pub fn get_groups(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_groups")
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |user: BasicUsername, _username: String, db: SqlitePool| async move {
                match db_get_user_groups(&db, &user.username).await {
                    Ok(groups) => {
                        log::info!("okay, got the groups: {:?}", &groups);
                        let group_names = db_get_all_group_names(&db, &user.username).await?;
                        let groups_struct = UserGroupsJson {
                            groups: group_names,
                        };
                        let json_resp = serde_json::to_string(&groups_struct)
                            .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
                        Ok(json_resp)
                    }
                    Err(_e) => Err(custom(WarpRejections::SqlxError(err_info!()))),
                }
            },
        )
}

pub fn create_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("create_group")
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_form: GroupForm, _username: String, db: SqlitePool| async move {
                let uuid = Uuid::new_v4();
                let uuid_string = uuid.to_string();
                // let group_data = GroupData::new(group_form.clone());
                let group_data = create_group_data(group_form.clone());
                let group_struct = DBGroupStruct {
                    id: uuid_string,
                    group_data,
                };
                match db_insert_group(&db, group_struct).await {
                    Ok(_) => {
                        let user_data = db_get_user(&db, &group_form.username).await?;
                        let group_info = GroupInfo {
                            uuid: uuid.to_string(),
                            name: group_form.group_name,
                        };
                        db_add_group_to_user(&db, user_data, group_info).await?;
                        Ok(warp::reply())
                    }
                    Err(e) => Err(e),
                }
            },
        )
}
pub fn authorize_refresh(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("refresh_auth")
        .and(warp::filters::header::header("authorization"))
        .map(|token: String| match verify_token(token) {
            Ok(claims) => {
                let username = claims.username;
                if let Ok(token_response) = generate_tokens(username, Token::Access) {
                    let code = StatusCode::OK;
                    let reply = warp::reply::json(&token_response);
                    warp::reply::with_status(reply, code)
                } else {
                    let code = StatusCode::BAD_REQUEST;
                    let reply = warp::reply::json(&ErrorMessage {
                        code: code.into(),
                        message: WarpRejections::AuthError(err_info!()).into(),
                    });
                    warp::reply::with_status(reply, code)
                }
            }
            Err(_) => {
                let code = StatusCode::UNAUTHORIZED;
                let reply = warp::reply::json(&ErrorMessage {
                    code: code.into(),
                    message: WarpRejections::AuthError(err_info!()).into(),
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
                    message: WarpRejections::AuthError(err_info!()).into(),
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
                Err(_e) => Err(custom(WarpRejections::AutocompleteError(err_info!()))),
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
            // let user_data = UserData::new(user_info.clone()).await?;
            let user_data = create_user_data(user_info.clone()).await?;
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
            let token_response = generate_tokens(user_info.0.clone(), Token::Refresh)?;
            log::info!("user_info: {:?}", &user_info);
            match verify_pass(user.password, user_info.1.salt, user_info.1.hashed_password)? {
                true => Ok(json(&token_response)),
                false => Err(custom(WarpRejections::AuthError(err_info!()))),
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
