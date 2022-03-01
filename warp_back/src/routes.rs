use crate::auth::{verify_pass, verify_token, with_auth};
use crate::err_info;
use crate::error_handling::WarpRejections;
use crate::new_db_stuff::db_update_group;
use crate::State;
use http::status::StatusCode;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::auth_structs::{ErrorMessage, Token, UserInfo};
use shared_stuff::db_structs::DBGroupStruct;
use shared_stuff::group_structs::{AddUser, GroupForm, GroupInfo};
use shared_stuff::imdb_structs::ImdbQuery;
use sqlx::types::uuid::Uuid;
use sqlx::SqlitePool;
use warp::reject::custom;

use crate::auth::generate_tokens;

use warp::reply::json;
use warp::Filter;

use crate::new_db_stuff::{
    create_group_data, create_user_data, db_add_user_to_group, db_get_user, db_insert_group,
    db_insert_user, db_update_user, db_user_leave_group, db_verify_group_member,
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
                match db_verify_group_member(group_id, username, &db).await {
                    Ok(group_struct) => {
                        let json_resp = serde_json::to_string(&group_struct)
                            .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
                        Ok(json_resp)
                    }
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn add_user_to_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("add_user")
        .and(warp::path::param())
        .and(warp::body::json())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_id: String, add_user: AddUser, _username: String, db: SqlitePool| async move {
                let add_username = add_user.username;
                match db_add_user_to_group(&group_id, &add_username, &db).await {
                    Ok(_) => Ok(warp::reply()),
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn leave_group(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("leave_group")
        .and(warp::path::param())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(
            |group_id: String, username: String, db: SqlitePool| async move {
                match db_user_leave_group(&db, username, &group_id).await {
                    Ok(_) => Ok(warp::reply()),
                    Err(e) => Err(e),
                }
            },
        )
}

pub fn update_group_data(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("update_group_data")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|group_struct: DBGroupStruct, db: SqlitePool| async move {
            match db_update_group(&db, &group_struct).await {
                Ok(_) => Ok(warp::reply()),
                Err(e) => {
                    log::info!("error is: {:?}", &e);
                    Err(e)
                }
            }
        })
}
pub fn get_all_groups(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("get_all_groups")
        .and(warp::path::param())
        .and(with_auth())
        .and(with_db(state.db.clone()))
        .and_then(|param_username: String, username: String, db: SqlitePool| async move {
            if !param_username.eq(&username) {
                return Err(custom(WarpRejections::SqlxError(err_info!())));
            }
            match db_get_user(&db, &username).await {
                Ok(user_struct) => {
                    let json_resp = serde_json::to_string(&user_struct.user_data.groups)
                        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
                    Ok(json_resp)
                }
                Err(_e) => Err(custom(WarpRejections::SqlxError(err_info!()))),
            }
        })
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
                let uuid_string = Uuid::new_v4().to_string();
                let group_data = create_group_data(&group_form);
                let group_struct = DBGroupStruct {
                    id: uuid_string.clone(),
                    group_data,
                };
                match db_insert_group(&db, group_struct).await {
                    Ok(_) => {
                        let mut user_struct = db_get_user(&db, &group_form.username).await?;
                        let group_info = GroupInfo {
                            uuid: uuid_string,
                            name: group_form.group_name,
                        };
                        user_struct.user_data.groups.insert(group_info);
                        db_update_user(&db, user_struct).await?;
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
            let user_struct = create_user_data(user_info.clone()).await?;
            match db_insert_user(&db, user_struct).await {
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
            let user_struct = db_get_user(&db, &user.username).await?;
            let token_response = generate_tokens(user_struct.username.clone(), Token::Refresh)?;
            //log::info!("user_info: {:?}", &user_struct);
            match verify_pass(
                user.password,
                user_struct.user_data.salt,
                user_struct.user_data.hashed_password,
            )? {
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
