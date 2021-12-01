use std::path::PathBuf;

pub fn load_logger() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    Ok(())
}
