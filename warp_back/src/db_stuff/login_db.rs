use crate::db_stuff::shared_db::acquire_db;
use crate::error_handling::{Result, SqlxError, WarpRejections};
use shared_stuff::LoginLookup;
use sqlx::query_as;
use sqlx::SqlitePool;
use warp::reject::custom;

pub async fn check_login(db: &SqlitePool, username: &str) -> Result<LoginLookup> {
    let mut conn = acquire_db(db).await?;
    let user_info = query_as!(
        LoginLookup,
        r#"
            select username, hashed_password, salt
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::CheckLoginError)))?;

    Ok(user_info)
}
