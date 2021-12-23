use crate::auth::verify_pass;
use shared_stuff::groups_stuff::GroupsId;

use crate::error_handling::{AuthError, Result, SqlxError, WarpRejections};
use serde_json::Value;
use shared_stuff::groups_stuff::BasicUsername;
use shared_stuff::groups_stuff::GroupForm;
use shared_stuff::LoginLookup;
use shared_stuff::UserInfo;
use sqlx::types::chrono::NaiveDateTime;
use sqlx::SqlitePool;
use sqlx::{query, query_as};
use uuid::Uuid;
use warp::reject::custom;

use crate::auth::hasher;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub salt: String, // hash helper
    pub groups: Option<String>,
    pub date_created: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

pub async fn update_user_group(db: &SqlitePool, username: &str, group_id: String) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let groups = GroupsId {
        groups: vec![group_id],
    };
    let serialized_groups =
        serde_json::to_string(&groups).map_err(|_| custom(WarpRejections::SerializationError))?;

    query!(
        r#"

                    UPDATE users 
                    set groups=$1
                    WHERE username=$2
                    "#,
        serialized_groups,
        username
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::CreateGroupError)))?;

    Ok(())
}

pub async fn create_new_group(db: &SqlitePool, group_form: &GroupForm) -> Result<String> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;

    let now = sqlx::types::chrono::Utc::now();
    let uuid = Uuid::new_v4().to_string();
    let serialized_users = serde_json::to_string(&vec![BasicUsername {
        username: group_form.username.to_string(),
    }])
    .map_err(|_| custom(WarpRejections::SerializationError))?;

    query!(
        r#"

        INSERT INTO groups ( id, name, members, movies_watched, current_movies, turn, date_created, date_modified  )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)

        "#,
        uuid,
        group_form.group_name,
        serialized_users,
        None::<Option<String>>,
        None::<Option<String>>,
        None::<Option<String>>,
        now,
        now
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::CreateGroupError)))?;

    Ok(uuid)
}

pub async fn select_single_user(db: &SqlitePool, username: &str) -> Result<User> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let user = query_as!(
        User,
        r#"
            select *
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::FetchUserError)))?;

    Ok(user)
}

pub async fn check_login(db: &SqlitePool, username: &str) -> Result<LoginLookup> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
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

pub async fn select_all_users(db: &SqlitePool) -> Result<Vec<User>> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let user = query_as!(
        User,
        r#"
        select * from users;
        "#
    )
    .fetch_all(&mut conn)
    .await
    .map_err(|_| {
        custom(WarpRejections::SqlxRejection(
            SqlxError::SelectAllUsersError,
        ))
    })?;

    Ok(user)
}

pub async fn update_password(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
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
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
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
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
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
