use crate::db_functions::{check_login, insert_user};
use crate::password_auth::verify_pass;
use crate::State;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::ImdbQuery;
use shared_stuff::UserInfo;
use sqlx::SqlitePool;
use std::collections::HashMap;

use warp::reply::json;
use warp::Filter;

// domain/search -> do seach()
// why bytes? some of these could def use json since requests come in as json

// getthe search string -> bytes
// use the bytes -> &str -> send to the autocomplete_func and get back the results
// results ok -> turn it to json and send it back to yew
// results bad -> 400 bad request. //TODO currently it's 404

#[derive(Debug)]
enum WarpRejections {
    SerializationError,
    UTF8Error,
}

impl warp::reject::Reject for WarpRejections {}

pub fn search(
    state: &State,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("search")
        .and(warp::body::json())
        .and_then(|query: ImdbQuery| async move {
            //log::info!("{:?}", &query);
            //let new = query.get(&String::from("query")).unwrap().clone();
            //let umm = ImdbQuery { query: new };
            if let Ok(movie_vec) = autocomplete_func(query).await {
                log::info!("{:?}", &movie_vec);
                let json_res = json(&movie_vec);
                Ok(json_res)
            } else {
                log::info!("error in autocomplete_func?");
                Err(warp::reject::not_found())
            }
        })
        .with(&state.cors)
}

pub fn register(
    state: &State,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|user: UserInfo, db: SqlitePool| async move {
            log::info!("{:?}", &user);
            match insert_user(&user, &db).await {
                Ok(e) => {
                    log::info!("bad user?");
                    Ok(warp::reply())
                }
                Err(e) => {
                    log::info!("err here");
                    Err(warp::reject::not_found())
                }
            }
        })
        .with(&state.cors)
}

pub fn login(
    state: &State,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::body::json())
        .and(with_db(state.db.clone()))
        .and_then(|user: UserInfo, db: SqlitePool| async move {
            //let user: UserInfo = serde_json::from_str(std::str::from_utf8(&u).unwrap()).unwrap();
            if let Ok(user_info) = check_login(&db, &user.username).await {
                match verify_pass(user.password, user_info.salt, user_info.hashed_password) {
                    true => {
                        log::info!("got it");
                        Ok(warp::reply())
                    }
                    false => {
                        log::info!("bad pass");
                        Err(warp::reject::not_found())
                    }
                }
            } else {
                log::info!("bad login");
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
