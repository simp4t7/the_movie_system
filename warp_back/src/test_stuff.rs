use crate::err_info;
use crate::error_handling::Result;
use crate::error_handling::WarpRejections;
use crate::{make_cors, State};
use sqlx::migrate::MigrateDatabase;
use sqlx::query;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use std::fs::remove_file;
use warp::reject::custom;

impl State {
    pub async fn test_init(db_name: &str) -> Result<Self> {
        let db = setup_new_db(db_name).await?;
        let cors = make_cors();
        Ok(Self { db, cors })
    }
}

pub fn delete_db(db_name: &str) -> Result<()> {
    let db_str = get_db_url(db_name)?;
    remove_file(&db_str).map_err(|_| custom(WarpRejections::Other(err_info!())))?;
    remove_file(format!("{}-shm", &db_str))
        .map_err(|_| custom(WarpRejections::Other(err_info!())))?;
    remove_file(format!("{}-wal", &db_str))
        .map_err(|_| custom(WarpRejections::Other(err_info!())))?;
    Ok(())
}

pub async fn init_db(db: &SqlitePool) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_e| custom(WarpRejections::SqlxError(err_info!())))?;
    query(
        r#"
            CREATE TABLE users
        (
            username TEXT NOT NULL,
            data TEXT NOT NULL
        );
"#,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub fn get_db_url(db_name: &str) -> Result<String> {
    let mut current_dir =
        std::env::current_dir().map_err(|_| custom(WarpRejections::Other(err_info!())))?;
    current_dir.push(db_name);
    let db_url = current_dir.into_os_string();
    let db_str = db_url
        .into_string()
        .map_err(|_| custom(WarpRejections::Other(err_info!())))?;
    Ok(db_str)
}

pub async fn setup_new_db(db_name: &str) -> Result<SqlitePool> {
    let db_str = get_db_url(db_name)?;
    if Sqlite::database_exists(&db_str)
        .await
        .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?
    {
        delete_db(db_name)?;
    }
    let _new_db = Sqlite::create_database(&db_str)
        .await
        .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;
    let pool = SqlitePool::connect(&db_str)
        .await
        .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;
    init_db(&pool).await?;

    Ok(pool)
}
