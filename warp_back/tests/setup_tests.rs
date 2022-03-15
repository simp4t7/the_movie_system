use scopeguard::defer;
use shared_stuff::auth_structs::UserInfo;
use sqlx::migrate::MigrateDatabase;
use sqlx::Sqlite;

use warp_back::error_handling::Result;
use warp_back::new_db_stuff::create_user_data;
use warp_back::new_db_stuff::{db_get_user, db_insert_user};
use warp_back::test_stuff::{delete_db, get_db_url, setup_new_db};

use ctor::ctor;
#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::try_init();
}

// Currently 2 options: each test creates and tearsdown its own DB and runs concurrently.
// Or, create a single test DB and add to it but run tests single threaded.
// For now I prefer to run them concurrently becuase serial doesn't work and
// test-threads=1 runs them in alphabetical order which is weird...

#[tokio::test]
// Check that the DB is created.
async fn create_db() -> Result<()> {
    let db_name = "test_db_1";
    let _new_db = setup_new_db(db_name).await?;
    assert!(Sqlite::database_exists(&get_db_url(db_name)?)
        .await
        .unwrap());
    delete_db(db_name)?;

    Ok(())
}

#[tokio::test]
// Check the DB still exists, and insert a user and check it was successful.
async fn insert_new_user() -> Result<()> {
    let db_name = "test_db_2";
    defer!(delete_db(db_name););
    let new_db = setup_new_db(db_name).await?;
    assert!(Sqlite::database_exists(&get_db_url(db_name)?)
        .await
        .unwrap());
    let new_user = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    let user_data = create_user_data(new_user.clone()).await?;
    log::info!("user_data: {:?}", &user_data);
    db_insert_user(&new_db, user_data).await?;
    log::info!("inserted");
    let check_user_data = db_get_user(&new_db, &new_user.username).await?;
    assert!(check_user_data.username == "Indiana".to_string());

    Ok(())
}
//#[tokio::test]
//// Try to insert a new user with the same username and assert that it fails.
//async fn duplicate_new_user() -> Result<()> {
//let db_name = "test_db_3";
//let new_db = setup_new_db(db_name).await?;
//let new_user = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user, &new_db).await?;
//let duplicate = insert_user(&new_user, &new_db).await;
//println!("{:?}", &duplicate);
//assert!(
//duplicate.unwrap_err().find::<WarpRejections>()
//== Some(&WarpRejections::SqlxRejection(SqlxError::InsertUserError))
//);
//let user_vec = select_all_users(&new_db).await?;
//assert!(user_vec.len() == 1);
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//// Make sure the inserted password is hashed
//async fn check_password_hash() -> Result<()> {
//let db_name = "test_db_4";
//let new_db = setup_new_db(db_name).await?;
//let new_user = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user, &new_db).await?;
//let login_info = check_login(&new_db, &new_user.username).await?;
//assert_ne!(login_info.hashed_password, new_user.password);
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//// Make sure 2 users with identical passwords have different password hashes.
//async fn check_same_passwords_different_hash() -> Result<()> {
//let db_name = "test_db_5";
//let new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_1, &new_db).await?;
//let new_user_2 = UserInfo {
//username: "Jones".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_2, &new_db).await?;

//let login_info_1 = check_login(&new_db, &new_user_1.username).await?;
//let login_info_2 = check_login(&new_db, &new_user_2.username).await?;
//assert_ne!(login_info_1.hashed_password, login_info_2.hashed_password);
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//// Make sure that date_modified is correctly applied when a user's info changes.
//// Means we also need to add an update function, like to change password or username.
//async fn check_date_modified() -> Result<()> {
//let db_name = "test_db_6";
//let new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//let new_user_2 = UserInfo {
//username: "Jones".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_1, &new_db).await?;
//update_username(&new_db, &new_user_1, &new_user_2).await?;
//let user = select_single_user(&new_db, &new_user_2.username).await?;
//assert_ne!(user.date_modified, user.date_created);
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//// Check to make sure usernames are valid e-mails.
//async fn check_usernames_are_valid_emails() -> Result<()> {
//let db_name = "test_db_7";
//let _new_db = setup_new_db(db_name).await?;
//assert!(validate_email("Indiana@jones.ark"));
//assert!(!validate_email("Indiana"));
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//async fn check_salts_are_different() -> Result<()> {
//let db_name = "test_db_8";
//let new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_1, &new_db).await?;
//let new_user_2 = UserInfo {
//username: "Jones".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_2, &new_db).await?;

//let login_info_1 = check_login(&new_db, &new_user_1.username).await?;
//let login_info_2 = check_login(&new_db, &new_user_2.username).await?;
//assert_ne!(login_info_1.salt, login_info_2.salt);
//delete_db(db_name)?;
//Ok(())
//}

//#[tokio::test]
//// Make sure the username and password validation functions are working.
//async fn check_validation_function() -> Result<()> {
//let db_name = "test_db_10";
//let _new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana@jones.ark".to_string(),
//password: "TheLostArKKK123!@#".to_string(),
//};
//assert!(new_user_1.validate().is_ok());

////TODO: add more tests for the validator.
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//// Check that the change username function works as expected
//async fn check_change_username() -> Result<()> {
//let db_name = "test_db_11";
//let new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//let new_user_2 = UserInfo {
//username: "Jones".to_string(),
//password: "password123".to_string(),
//};
//insert_user(&new_user_1, &new_db).await?;
//update_username(&new_db, &new_user_1, &new_user_2).await?;
//let user = select_single_user(&new_db, &new_user_2.username).await?;
//assert!(user.username == new_user_2.username);
//assert!(user.username != new_user_1.username);
//delete_db(db_name)?;
//Ok(())
//}

//#[tokio::test]
//// Check that the change password function works as expected
//async fn check_change_password() -> Result<()> {
//let db_name = "test_db_12";
//let new_db = setup_new_db(db_name).await?;
//let new_user_1 = UserInfo {
//username: "Indiana".to_string(),
//password: "password123".to_string(),
//};
//let new_user_2 = UserInfo {
//username: "Indiana".to_string(),
//password: "ummm378!".to_string(),
//};
//insert_user(&new_user_1, &new_db).await?;
//let not_updated = select_single_user(&new_db, &new_user_1.username).await?;
//update_password(&new_db, &new_user_1, &new_user_2).await?;
//let updated = select_single_user(&new_db, &new_user_2.username).await?;
//assert!(not_updated.hashed_password != updated.hashed_password);
//assert!(not_updated.username == updated.username);
//delete_db(db_name)?;
//Ok(())
//}
//#[tokio::test]
//async fn test_format() -> Result<(), Box<dyn std::error::Error>> {
//let db_name = "test_db_13";
//let new_db = setup_new_db(db_name).await?;
//todo!();
//delete_db(db_name);
//Ok(())
//}
