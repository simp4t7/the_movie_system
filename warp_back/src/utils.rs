use crate::error_handling::Result;
use crate::error_handling::WarpRejections;
use dotenv;
use pretty_env_logger;
use warp::reject::custom;

pub fn load_logger() -> Result<()> {
    dotenv::dotenv().map_err(|_| custom(WarpRejections::EnvError))?;
    pretty_env_logger::init();
    Ok(())
}
