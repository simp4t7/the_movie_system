use ctor::dtor;
use shared_stuff::UserInfo;
use sqlx::migrate::MigrateDatabase;
use sqlx::query;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use std::fs::remove_file;
use tokio::sync::OnceCell;
use validator::validate_email;
use validator::Validate;
use warp_back::db_functions::{check_login, insert_user, select_all_users};

fn delete_db(db_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let db_str = get_db_url(db_name)?;
    remove_file(&db_str)?;
    remove_file(format!("{}-shm", &db_str))?;
    remove_file(format!("{}-wal", &db_str))?;
    Ok(())
}

async fn init_db(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = db.acquire().await?;
    query(
        r#"
            CREATE TABLE users
        (
            id TEXT NOT NULL,
            username TEXT NOT NULL UNIQUE,
            hashed_password TEXT not null,
            salt TEXT not null,
            date_created DATETIME with time zone not null,
            date_modified TIMESTAMP with time zone not null
        );
"#,
    )
    .execute(&mut conn)
    .await?;

    Ok(())
}

fn get_db_url(db_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push(db_name);
    let db_url = current_dir.into_os_string();
    let db_str = db_url
        .into_string()
        .map_err(|e| format!("problem with OsString: {:?}", e))?;
    Ok(db_str)
}

async fn setup_new_db(db_name: &str) -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let db_str = get_db_url(db_name)?;
    if Sqlite::database_exists(&db_str).await? {
        delete_db(db_name);
    }
    let _new_db = Sqlite::create_database(&db_str).await?;
    let pool = SqlitePool::connect(&db_str).await.unwrap();
    init_db(&pool).await?;

    Ok(pool)
}

// Currently 2 options: each test creates and tearsdown its own DB and runs concurrently.
// Or, create a single test DB and add to it but run tests single threaded.
// For now I prefer to run them concurrently becuase serial doesn't work and
// test-threads=1 runs them in alphabetical order which is weird...

#[tokio::test]
// Check that the DB is created.
async fn create_db() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_1";
    let new_db = setup_new_db(db_name).await?;
    assert!(Sqlite::database_exists(&get_db_url(db_name)?).await?);
    delete_db(db_name);

    Ok(())
}

#[tokio::test]
// Check the DB still exists, and insert a user and check it was successful.
async fn insert_new_user() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_2";
    let new_db = setup_new_db(db_name).await?;
    assert!(Sqlite::database_exists(&get_db_url(db_name)?).await?);
    let new_user = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user, &new_db).await?;
    let user_vec = select_all_users(&new_db).await?;
    assert!(user_vec.len() == 1);
    assert!(user_vec[0].username == "Indiana".to_string());
    delete_db(db_name);

    Ok(())
}
#[tokio::test]
// Try to insert a new user with the same username and assert that it fails.
async fn duplicate_new_user() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_3";
    let new_db = setup_new_db(db_name).await?;
    let new_user = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user, &new_db).await?;
    assert!(insert_user(&new_user, &new_db).await.is_err());
    let user_vec = select_all_users(&new_db).await?;
    assert!(user_vec.len() == 1);
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
// Make sure the inserted password is hashed
async fn check_password_hash() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_4";
    let new_db = setup_new_db(db_name).await?;
    let new_user = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user, &new_db).await?;
    let login_info = check_login(&new_db, new_user.username).await?;
    assert_ne!(login_info.hashed_password, new_user.password);
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
// Make sure 2 users with identical passwords have different password hashes.
async fn check_same_passwords_different_hash() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_5";
    let new_db = setup_new_db(db_name).await?;
    let new_user_1 = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user_1, &new_db).await?;
    let new_user_2 = UserInfo {
        username: "Jones".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user_2, &new_db).await?;

    let login_info_1 = check_login(&new_db, new_user_1.username).await?;
    let login_info_2 = check_login(&new_db, new_user_2.username).await?;
    assert_ne!(login_info_1.hashed_password, login_info_2.hashed_password);
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
// Make sure that date_modified is correctly applied when a user's info changes.
// Means we also need to add an update function, like to change password or username.
async fn check_date_modified() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_6";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
// Check to make sure usernames are valid e-mails.
async fn check_usernames_are_valid_emails() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_7";
    let new_db = setup_new_db(db_name).await?;
    assert!(validate_email("Indiana@jones.ark"));
    assert!(validate_email("Indiana") == false);
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_salts_are_different() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_8";
    let new_db = setup_new_db(db_name).await?;
    let new_user_1 = UserInfo {
        username: "Indiana".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user_1, &new_db).await?;
    let new_user_2 = UserInfo {
        username: "Jones".to_string(),
        password: "password123".to_string(),
    };
    insert_user(&new_user_2, &new_db).await?;

    let login_info_1 = check_login(&new_db, new_user_1.username).await?;
    let login_info_2 = check_login(&new_db, new_user_2.username).await?;
    assert_ne!(login_info_1.salt, login_info_2.salt);
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_error_message_on_duplicate_user() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_9";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_valid_utf8() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_10";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_min_username_len() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_11";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_max_username_len() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_12";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_min_password_len() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_13";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
#[tokio::test]
async fn check_max_password_len() -> Result<(), Box<dyn std::error::Error>> {
    let db_name = "test_db_14";
    let new_db = setup_new_db(db_name).await?;
    todo!();
    delete_db(db_name);
    Ok(())
}
