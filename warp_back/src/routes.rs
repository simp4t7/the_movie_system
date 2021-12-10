use crate::auth::verify_pass;
use crate::db_functions::{check_login, insert_user};
use crate::error_handling::WarpRejections;
use crate::State;
use imdb_autocomplete::autocomplete_func;
use shared_stuff::ImdbQuery;
use shared_stuff::UserInfo;
use sqlx::SqlitePool;
use warp::reject::custom;
use warp::reply::Reply;

use crate::auth::generate_jwt;
use crate::error_handling::AuthError;
use crate::error_handling::SqlxError;
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
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("search")
        .and(warp::body::json())
        .and_then(|query: ImdbQuery| async move {
            if let Ok(movie_vec) = autocomplete_func(query).await {
                log::info!("{:?}", &movie_vec);
                let json_res = json(&movie_vec);
                Ok(json_res)
            } else {
                log::info!("error in autocomplete_func?");
                Err(custom(WarpRejections::AutocompleteError))
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
                Ok(_e) => Ok(warp::reply()),
                Err(_e) => Err(custom(WarpRejections::SqlxRejection(
                    SqlxError::InsertUserError,
                ))),
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
            let user_info = check_login(&db, &user.username).await?;
            let token = generate_jwt(user_info.username.clone())?;
            log::info!("user_info: {:?}", &user_info);
            match verify_pass(user.password, user_info.salt, user_info.hashed_password)? {
                true => Ok(
                    warp::reply::with_header(warp::reply(), "authorization", token).into_response(),
                ),
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
