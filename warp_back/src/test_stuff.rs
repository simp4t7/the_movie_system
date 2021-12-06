use crate::{make_cors, State};
use anyhow::anyhow;
use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::query;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use std::fs::remove_file;

impl State {
    pub async fn test_init(db_name: &str) -> Result<Self> {
        let db = setup_new_db(db_name).await?;
        let cors = make_cors();
        Ok(Self { db, cors })
    }
}

pub fn delete_db(db_name: &str) -> Result<()> {
    let db_str = get_db_url(db_name)?;
    remove_file(&db_str)?;
    remove_file(format!("{}-shm", &db_str))?;
    remove_file(format!("{}-wal", &db_str))?;
    Ok(())
}

pub async fn init_db(db: &SqlitePool) -> Result<()> {
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

pub fn get_db_url(db_name: &str) -> Result<String> {
    let mut current_dir = std::env::current_dir()?;
    current_dir.push(db_name);
    let db_url = current_dir.into_os_string();
    let db_str = db_url
        .into_string()
        .map_err(|e| anyhow!("problem with OsString: {:?}", e))?;
    Ok(db_str)
}

pub async fn setup_new_db(db_name: &str) -> Result<SqlitePool> {
    let db_str = get_db_url(db_name)?;
    if Sqlite::database_exists(&db_str).await? {
        delete_db(db_name)?;
    }
    let _new_db = Sqlite::create_database(&db_str).await?;
    let pool = SqlitePool::connect(&db_str).await.unwrap();
    init_db(&pool).await?;

    Ok(pool)
}
