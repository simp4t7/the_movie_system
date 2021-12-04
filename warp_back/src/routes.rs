use crate::db_functions::{check_login, insert_user};
use crate::make_cors;
use crate::password_auth::verify_pass;
use crate::State;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::UserInfo;
use sqlx::SqlitePool;
use std::sync::Arc;
use warp::reply::json;
use warp::Filter;

// domain/search -> do seach()
// why bytes? some of these could def use json since requests come in as json

// getthe search string -> bytes
// use the bytes -> &str -> send to the autocomplete_func and get back the results
// results ok -> turn it to json and send it back to yew
// results bad -> 400 bad request. //TODO currently it's 404

pub fn search(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("search")
        .and(warp::body::bytes())
        .and_then(|b: bytes::Bytes| async move {
            if let Ok(movie_vec) = autocomplete_func(std::str::from_utf8(&b).unwrap()).await {
                log::info!("{:?}", &movie_vec);
                let json_res = json(&movie_vec);
                Ok(json_res)
            } else {
                Err(warp::reject::not_found())
            }
        })
        .with(make_cors())
}

pub fn register(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::body::bytes())
        .and(with_db(state.db.clone()))
        .and_then(|u: bytes::Bytes, db: SqlitePool| async move {
            let user: UserInfo = serde_json::from_str(std::str::from_utf8(&u).unwrap()).unwrap();
            log::info!("{:?}", &user);
            if insert_user(&user, &db).await.is_ok() {
                Ok(warp::reply())
            } else {
                Err(warp::reject::not_found())
            }
        })
        .with(&state.cors)
}

pub fn login(
    state: &State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::body::bytes())
        .and(with_db(state.db.clone()))
        .and_then(|u: bytes::Bytes, db: SqlitePool| async move {
            let user: UserInfo = serde_json::from_str(std::str::from_utf8(&u).unwrap()).unwrap();
            if let Ok(user_info) = check_login(&db, &user.username).await {
                match verify_pass(user.password, user_info.salt, user_info.hashed_password) {
                    true => Ok(warp::reply()),
                    false => Err(warp::reject::not_found()),
                }
            } else {
                Err(warp::reject::not_found())
            }
        })
        .with(&state.cors)
}

fn with_db(
    db: SqlitePool,
) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
