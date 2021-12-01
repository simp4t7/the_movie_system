use shared_stuff::utils::load_logger;
use warp::Filter;
use warp_back::db_functions::make_db_pool;

use warp_back::routes::login;
use warp_back::routes::register;
use warp_back::routes::search;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    load_logger();
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
    let cors = warp::cors().allow_any_origin().build();
    let db_pool = make_db_pool().await.unwrap();

    let routes = search(&cors).or(login(&cors)).or(register(&cors));

    warp::serve(routes).bind(([0, 0, 0, 0], 3030)).await;
}
