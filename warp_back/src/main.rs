use anyhow::anyhow;
use anyhow::Result;
use shared_stuff::utils::load_logger;

use warp::Filter;

use warp_back::routes::login;
use warp_back::routes::register;
use warp_back::routes::search;
use warp_back::State;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    load_logger().map_err(|e| anyhow!("problem loading logger: {:?}", e))?;
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
    //let state = Arc::new(State::init().await);
    let state = State::init().await;

    let routes = search(&state).or(login(&state)).or(register(&state));

    warp::serve(routes).bind(([0, 0, 0, 0], 3030)).await;
    Ok(())
}
