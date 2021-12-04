use anyhow::anyhow;
use anyhow::Error;
use anyhow::Result;

use crate::password_auth::verify_pass;

use shared_stuff::LoginLookup;
use shared_stuff::UserInfo;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::password_auth::hasher;

#[derive(Debug)]
pub enum CustomError<'a> {
    DBError(&'a str),
    Other(&'a str),
}

impl<'a> std::error::Error for CustomError<'a> {}

impl<'a> std::fmt::Display for CustomError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CustomError::DBError(msg) => write!(f, "{}", msg),
            CustomError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub salt: String, // hash helper
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

// what we get back from after looking up the login in the db and then we verify

pub fn map_sqlite_err(err: sqlx::Error) -> Error {
    if let Some(code) = err.into_database_error().unwrap().code() {
        match &*code {
            "2067" => anyhow!("username already taken, choose another"),
            _ => anyhow!("umm, another error?"),
        }
    } else {
        unreachable!();
    }
}

pub async fn select_single_user(db_pool: &SqlitePool, username: &str) -> Result<User> {
    let user = query_as!(
        User,
        r#"
            select *
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(db_pool)
    .await?;

    Ok(user)
}

pub async fn check_login(db_pool: &SqlitePool, username: &str) -> Result<LoginLookup> {
    let user_info = query_as!(
        LoginLookup,
        r#"
            select username, hashed_password, salt
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(db_pool)
    .await?;

    Ok(user_info)
}

pub async fn select_all_users(db: &SqlitePool) -> Result<Vec<User>> {
    let _conn = db.acquire().await?;
    let user = query_as!(
        User,
        r#"
        select * from users;
        "#
    )
    .fetch_all(db)
    .await?;
    Ok(user)
}

pub async fn update_password(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let mut conn = db.acquire().await?;
    let now = sqlx::types::chrono::Utc::now();
    if let Ok(user_info) = check_login(db, &old_user.username).await {
        match verify_pass(
            old_user.password.clone(),
            user_info.salt,
            user_info.hashed_password,
        ) {
            true => {
                let (new_hashed_password, new_salt) = hasher(&new_user.password).await?;
                query!(
                    r#"

                    UPDATE users 
                    set hashed_password=$1,
                    salt=$2,
                    date_modified=$3
                    WHERE username=$4;

                    "#,
                    new_hashed_password,
                    new_salt,
                    now,
                    old_user.username,
                )
                .execute(&mut conn)
                .await?;
            }
            false => {
                anyhow!("incorrect password, cannot update username");
            }
        }
    } else {
        anyhow!("user is not in the database");
    }
    Ok(())
}

pub async fn update_username(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let mut conn = db.acquire().await?;
    let now = sqlx::types::chrono::Utc::now();
    if let Ok(user_info) = check_login(db, &old_user.username).await {
        match verify_pass(
            old_user.password.clone(),
            user_info.salt,
            user_info.hashed_password,
        ) {
            true => {
                query!(
                    r#"

                    UPDATE users 
                    set username=$1,
                    date_modified=$2
                    WHERE username=$3;

                    "#,
                    new_user.username,
                    now,
                    old_user.username,
                )
                .execute(&mut conn)
                .await?;
            }
            false => {
                anyhow!("incorrect password, cannot update username");
            }
        }
    } else {
        anyhow!("user is not in the database");
    }
    Ok(())
}

pub async fn insert_user(user: &UserInfo, db: &SqlitePool) -> Result<()> {
    let mut conn = db.acquire().await?;
    let now = sqlx::types::chrono::Utc::now();
    let uuid = Uuid::new_v4().to_string();
    let (password_hash, salt) = hasher(&user.password).await?;
    query!(
        r#"

        INSERT INTO users ( id, username, hashed_password, salt, date_created, date_modified  )
        VALUES ($1, $2, $3, $4, $5, $6)

        "#,
        uuid,
        user.username,
        password_hash,
        salt,
        now,
        now
    )
    .execute(&mut conn)
    .await
    .map_err(map_sqlite_err)?;

    let _users: Vec<User> = select_all_users(db).await?;

    Ok(())
}
