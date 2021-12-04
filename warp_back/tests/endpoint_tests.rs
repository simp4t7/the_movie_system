use anyhow::Result;
use serde_json::to_string;
use shared_stuff::{LoginLookup, UserInfo};
use warp_back::routes::{login, register, search};

#[tokio::test]
async fn check_search() -> Result<()> {
    let cors = warp::cors().allow_any_origin().build();
    let filter = search(&cors);
    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .body("Dune")
        .matches(&filter)
        .await;
    assert!(req);

    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .body("><#@")
        .matches(&filter)
        .await;
    assert!(!req);
    Ok(())
}

#[tokio::test]
async fn check_login() -> Result<()> {
    let state = State::init().await;
    let filter = login(&state);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/login")
        .body(serde_json::to_string(&new_user_1)?)
        .matches(&filter)
        .await;
    println!("{:?}", &req);

    let new_user_2 = LoginLookup {
        username: "Indiana@Jones.ok".to_string(),
        hashed_password: "umm237@#".to_string(),
        salt: "salty".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .body(serde_json::to_string(&new_user_2)?)
        .matches(&filter)
        .await;
    println!("{:?}", &req);

    Ok(())
}
