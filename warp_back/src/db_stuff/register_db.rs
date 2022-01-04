use crate::auth::{hasher, verify_pass};
use crate::db_stuff::login_db::check_login;
use crate::db_stuff::shared_db::acquire_db;
use crate::error_handling::{AuthError, Result, SqlxError, WarpRejections};
use shared_stuff::UserInfo;
use sqlx::{query, SqlitePool};
use uuid::Uuid;
use warp::reject::custom;

pub async fn update_password(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let now = sqlx::types::chrono::Utc::now();
    if let Ok(user_info) = check_login(db, &old_user.username).await {
        match verify_pass(
            old_user.password.clone(),
            user_info.salt,
            user_info.hashed_password,
        )? {
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
                .await
                .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
            }
            false => {
                custom(WarpRejections::AuthRejection(AuthError::VerifyError));
            }
        }
    } else {
        custom(WarpRejections::SqlxRejection(SqlxError::CheckLoginError));
    }
    Ok(())
}

pub async fn update_username(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let now = sqlx::types::chrono::Utc::now();
    if let Ok(user_info) = check_login(db, &old_user.username).await {
        match verify_pass(
            old_user.password.clone(),
            user_info.salt,
            user_info.hashed_password,
        )? {
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
                .await
                .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
            }
            false => {
                custom(WarpRejections::AuthRejection(AuthError::VerifyError));
            }
        }
    } else {
        custom(WarpRejections::SqlxRejection(SqlxError::CheckLoginError));
    }
    Ok(())
}

pub async fn insert_user(user: &UserInfo, db: &SqlitePool) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let now = sqlx::types::chrono::Utc::now();
    let uuid = Uuid::new_v4().to_string();
    let (password_hash, salt) = hasher(&user.password).await?;
    query!(
        r#"

        INSERT INTO users ( id, username, hashed_password, salt, groups, date_created, date_modified  )
        VALUES ($1, $2, $3, $4, $5, $6, $7)

        "#,
        uuid,
        user.username,
        password_hash,
        salt,
        None::<Option<String>>,
        now,
        now
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::InsertUserError)))?;

    //let _users: Vec<User> = select_all_users(db).await?;

    Ok(())
}
