use shared_stuff::MovieInfo;
use shared_stuff::{ErrorMessage, ImdbQuery, LoginLookup, UserInfo};
use warp::Filter;
use warp_back::error_handling::handle_rejection;
use warp_back::error_handling::Result;

use warp_back::routes::{login, register, search};
use warp_back::test_stuff::delete_db;
use warp_back::State;

use ctor::ctor;
#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}

#[tokio::test]
// Check that the search function works, and returns a 200 status code on a good
// search, and a client error on a bad search. Ideally need better error handling
// and testing all the error variants.
async fn check_search() -> Result<()> {
    let db_name = "search_db";
    let state = State::test_init(db_name).await?;
    let filter = search(&state).recover(handle_rejection);
    let query: ImdbQuery = "Dune".into();

    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .json(&query)
        .reply(&filter)
        .await;
    assert!(req.status().is_success());
    let resp: Vec<MovieInfo> = serde_json::from_slice(&req.into_body()).unwrap();
    assert!(!resp.is_empty());

    let query: ImdbQuery = "><#@".into();
    let req = warp::test::request()
        .method("POST")
        .path("/search")
        .json(&query)
        .reply(&filter)
        .await;
    let resp: ErrorMessage = serde_json::from_slice(&req.into_body()).unwrap();
    assert!(resp.message == "AutocompleteError");

    delete_db(db_name)?;
    Ok(())
}

#[tokio::test]
// Check that the register endpoint works, and can successfully register a new user,
// and make sure that there's an error trying to register bad Json. Should do more here.
//
// Would be nice to get a proper BodyDeserializeError but kind of a pain because it happens
// in Warp's .body() method.
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

    //println!("{:?}", &req);
    assert!(req.status().is_client_error());

    delete_db(db_name)?;

    Ok(())
}
#[tokio::test]
// Check that the login works
// Check that wrong pass is a 400 error.
async fn check_login() -> Result<()> {
    let db_name = "login_db";
    let state = State::test_init(db_name).await?;
    let register = register(&state);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_1)
        .reply(&register)
        .await;
    assert!(req.status().as_u16() == 200);

    let login = login(&state).recover(handle_rejection);
    let req = warp::test::request()
        .method("POST")
        .path("/login")
        .json(&new_user_1)
        .reply(&login)
        .await;
    assert!(req.status().as_u16() == 200);

    let new_user_2 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "not_the_password".to_string(),
    };

    let req = warp::test::request()
        .method("POST")
        .path("/login")
        .json(&new_user_2)
        .reply(&login)
        .await;

    let resp: ErrorMessage = serde_json::from_slice(&req.into_body()).unwrap();
    println!("{:?}", &resp);
    assert!(resp.message == "AuthRejection(VerifyError)");

    delete_db(db_name)?;

    Ok(())
}

#[tokio::test]
// Check that can't register twice with the same account name
async fn check_register_twice() -> Result<()> {
    let db_name = "register_test_2";
    let state = State::test_init(db_name).await?;

    let register = register(&state).recover(handle_rejection);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };
    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_1)
        .reply(&register)
        .await;
    assert!(req.status().as_u16() == 200);

    let req = warp::test::request()
        .method("POST")
        .path("/register")
        .json(&new_user_1)
        .reply(&register)
        .await;

    let resp: ErrorMessage = serde_json::from_slice(&req.into_body()).unwrap();
    assert!(resp.message == "SqlxRejection(InsertUserError)");

    delete_db(db_name)?;
    Ok(())
}

#[tokio::test]
// Try to login without a registered user.
async fn check_bad_login() -> Result<()> {
    let db_name = "login_test_2";
    let state = State::test_init(db_name).await?;

    let login = login(&state).recover(handle_rejection);
    let new_user_1 = UserInfo {
        username: "Indiana@Jones.ok".to_string(),
        password: "umm237@#".to_string(),
    };

    let req = warp::test::request()
        .method("POST")
        .path("/login")
        .json(&new_user_1)
        .reply(&login)
        .await;

    let resp: ErrorMessage = serde_json::from_slice(&req.into_body()).unwrap();
    assert!(resp.message == "SqlxRejection(CheckLoginError)");
    delete_db(db_name)?;

    Ok(())
}
