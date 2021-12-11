#![cfg(not(target_arch = "wasm32"))]
// Pretty cool, this stops this being compiled for Yew, where it doesn't work.
use ctor::ctor;

#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}
