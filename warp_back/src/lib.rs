use crate::error_handling::Result;
use crate::error_handling::SqlxError;
use crate::error_handling::WarpRejections;
use dotenv::dotenv;
use dotenv::var;
use lazy_static::lazy_static;
use sqlx::SqlitePool;

use warp::cors::Cors;
use warp::reject::custom;

pub mod auth;
//pub mod db_stuff;
pub mod error_handling;
pub mod new_db_stuff;
pub mod routes;
pub mod test_stuff;

lazy_static! {
    static ref CORS_ORIGIN: String = {
        dotenv().ok();
        dotenv::var("CORS_ORIGIN").expect("env error")
    };
    static ref TOKEN_SECRET: String = {
        dotenv().ok();
        dotenv::var("DEV_SECRET").expect("env error")
    };
    pub static ref ACCESS_EXP: i64 = {
        dotenv().ok();
        dotenv::var("ACCESS_TOKEN_EXP")
            .expect("env error")
            .parse::<i64>()
            .expect("parse error")
    };
    pub static ref REFRESH_EXP: i64 = {
        dotenv().ok();
        dotenv::var("REFRESH_TOKEN_EXP")
            .expect("env error")
            .parse::<i64>()
            .expect("parse error")
    };
}

#[derive(Clone)]
pub struct State {
    pub db: SqlitePool,
    pub cors: Cors,
}

impl State {
    pub async fn init() -> Result<Self> {
        let db = make_db_pool().await?;
        let cors = make_cors();
        Ok(Self { db, cors })
    }
}

pub fn make_cors() -> Cors {
    log::info!("CORS_ORIGIN: {:?}", &*CORS_ORIGIN);
    warp::cors()
        .allow_headers(vec!["content-type", "authorization"])
        .allow_methods(&[
            warp::http::Method::GET,
            warp::http::Method::POST,
            warp::http::Method::OPTIONS,
        ])
        .allow_origin(CORS_ORIGIN.as_str())
        .allow_credentials(true)
        .expose_header("authorization")
        .build()
}

pub async fn make_db_pool() -> Result<SqlitePool> {
    dotenv().map_err(|_| custom(WarpRejections::EnvError))?;
    let pool =
        SqlitePool::connect(&var("DATABASE_URL").map_err(|_| custom(WarpRejections::EnvError))?)
            .await
            .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    Ok(pool)
}
