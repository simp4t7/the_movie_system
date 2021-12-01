use dotenv::dotenv;
use dotenv::var;
use shared_stuff::UserInfo;
use sqlx::types::chrono::DateTime;
use sqlx::types::chrono::Utc;
use sqlx::SqlitePool;
use sqlx::{query, query_as};
use uuid::Uuid;

use crate::password_auth::hasher;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub salt: String, // hash helper
    pub date_created: String,
    pub date_modified: String,
}

// what we get back from after looking up the login in the db and then we verify
pub struct LoginLookup {
    pub username: String,
    pub hashed_password: String,
    pub salt: String,
}

pub async fn make_db_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    dotenv();

    let pool = SqlitePool::connect(&var("DATABASE_URL")?).await.unwrap();
    Ok(pool)
}

pub async fn check_login(
    db_pool: &SqlitePool,
    username: String,
) -> Result<LoginLookup, Box<dyn std::error::Error>> {
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

pub async fn select_all_users(db: &SqlitePool) -> Result<Vec<User>, Box<dyn std::error::Error>> {
    let mut conn = db.acquire().await?;
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

pub async fn insert_user(
    user: &UserInfo,
    db: &SqlitePool,
) -> Result<(), Box<dyn std::error::Error>> {
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
    .await?;

    let users: Vec<User> = select_all_users(db).await?;

    Ok(())
}
