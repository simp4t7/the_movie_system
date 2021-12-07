use shared_stuff::{ImdbQuery, LoginLookup, UserInfo};
use warp_back::error_handling::Result;
use warp_back::error_handling::WarpRejections;
use warp_back::routes::{login, register, search};
use warp_back::test_stuff::delete_db;
use warp_back::State;

#[tokio::test]
// Check that the search function works, and returns a 200 status code on a good
// search, and a client error on a bad search. Ideally need better error handling
// and testing all the error variants.
async fn check_search() -> Result<()> {
    let db_name = "search_db";
    let state = State::test_init(db_name).await?;
    let filter = search(&state);
    let query: ImdbQuery = "Dune".into();

    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .json(&query)
        .reply(&filter)
        .await;
    assert!(req.status().is_success());

    let query: ImdbQuery = "><#@".into();
    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .json(&query)
        .reply(&filter)
        .await;
    assert!(req.status().is_client_error());

    delete_db(db_name)?;
    Ok(())
}

#[tokio::test]
// Check that the register endpoint works, and can successfully register a new user,
// and make sure that there's an error trying to register bad Json. Should do more here.
async fn check_register() -> Result<()> {
    let db_name = "register_db";
    let state = State::test_init(db_name).await?;
    let filter = register(&state);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_1)
        .reply(&filter)
        .await;
    assert!(req.status().is_success());

    let new_user_2 = LoginLookup {
        username: "Indiana@Jones.ok".to_string(),
        hashed_password: "umm237@#".to_string(),
        salt: "salty".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_2)
        .reply(&filter)
        .await;
    println!("{:?}", &req);
    //assert!(req.into_body().is_err());

    delete_db(db_name)?;

    Ok(())
}
#[tokio::test]
// Check that the login works
async fn check_login() -> Result<()> {
    let db_name = "login_db";
    let state = State::test_init(db_name).await?;
    let filter = register(&state);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_1)
        .reply(&filter)
        .await;
    assert!(req.status().as_u16() == 200);

    let login_filter = login(&state);
    let req = warp::test::request()
        .method("POST")
        .path("/login")
        .json(&new_user_1)
        .reply(&login_filter)
        .await;
    assert!(req.status().as_u16() == 200);

    delete_db(db_name)?;

    Ok(())
}
