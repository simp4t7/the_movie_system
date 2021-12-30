use warp::Filter;

use warp_back::error_handling::handle_rejection;
use warp_back::error_handling::Result;

use warp_back::routes::{add_user_to_group, create_group, get_groups, leave_group};
use warp_back::routes::{authorize_access, authorize_refresh, login, register, search};
use warp_back::State;

use log::{debug, error, info, trace, warn};

#[tokio::main]
async fn main() -> Result<()> {
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");

    let state = State::init().await?;

    let routes = search(&state)
        .or(register(&state))
        .or(login(&state))
        .or(authorize_access(&state))
        .or(authorize_refresh(&state))
        .or(create_group(&state))
        .or(leave_group(&state))
        .or(get_groups(&state))
        .or(add_user_to_group(&state))
        .recover(handle_rejection)
        .with(&state.cors);

    warp::serve(routes).bind(([0, 0, 0, 0], 3030)).await;
    Ok(())
}
