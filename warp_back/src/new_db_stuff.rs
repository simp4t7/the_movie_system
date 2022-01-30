use crate::auth::{hasher, verify_pass};

use crate::err_info;
use crate::error_handling::{Result, WarpRejections};
use shared_stuff::auth_structs::UserInfo;
use shared_stuff::db_structs::{DBGroup, DBGroupStruct, DBUser, DBUserStruct, GroupData, UserData};
use shared_stuff::group_structs::{GroupForm, GroupInfo};
use shared_stuff::shared_structs::SystemState;
use sqlx::pool::PoolConnection;
use sqlx::types::uuid::Uuid;
use sqlx::Sqlite;
use sqlx::{query, query_as, SqlitePool};
use std::collections::{HashMap, HashSet, VecDeque};
//use uuid::Uuid;
use warp::reject::custom;

pub async fn db_verify_group_member(
    group_id: String,
    username: String,
    db: &SqlitePool,
) -> Result<DBGroupStruct> {
    let group_struct = db_get_group(db, &group_id).await?;
    let members = &group_struct.group_data.members;
    if members.contains(&username) {
        Ok(group_struct)
    } else {
        Err(custom(WarpRejections::UserNotInGroup(err_info!())))
    }
}

pub async fn db_add_user_to_group(group_id: &str, new_member: &str, db: &SqlitePool) -> Result<()> {
    let mut group_struct = db_get_group(db, group_id).await?;

    // Needs to fail if the user doesn't exist. This handles it, but the order matters.
    let mut user_struct = db_get_user(db, &new_member).await?;
    group_struct
        .group_data
        .members
        .insert(new_member.to_string());

    db_update_group(db, &group_struct).await?;
    let group_info = GroupInfo {
        uuid: group_id.to_string(),
        name: group_struct.group_data.group_name.clone(),
    };
    user_struct.user_data.groups.insert(group_info);
    db_update_user(db, user_struct).await?;
    Ok(())
}

pub async fn db_user_leave_group(db: &SqlitePool, username: String, group_id: &str) -> Result<()> {
    let mut group_struct = db_get_group(db, group_id).await?;
    let mut user_struct = db_get_user(db, &username).await?;
    let group_name = &group_struct.group_data.group_name;

    let remove_group = GroupInfo {
        uuid: group_id.to_string(),
        name: group_name.to_string(),
    };

    // These return bools, so can match on them if you want to handle errors removing.
    user_struct.user_data.groups.remove(&remove_group);
    group_struct.group_data.members.remove(&username);
    db_update_user(db, user_struct).await?;

    match group_struct.group_data.members.is_empty() {
        true => db_delete_group(db, &group_id).await?,
        false => db_update_group(db, &group_struct).await?,
    }
    Ok(())
}

pub async fn create_user_data(user_info: UserInfo) -> Result<DBUserStruct> {
    let id = Uuid::new_v4().to_string();
    let username = user_info.username;
    let (hashed_password, salt) = hasher(&user_info.password).await?;
    let groups = HashSet::new();
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let user_data = UserData {
        id,
        hashed_password,
        salt,
        groups,
        date_created: now,
        date_modified: now,
    };
    let user_struct = DBUserStruct {
        username,
        user_data,
    };
    Ok(user_struct)
}

pub fn create_group_data(input: &GroupForm) -> GroupData {
    let members = HashSet::from([input.username.clone()]);
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let turn = String::from("");
    GroupData {
        group_name: input.group_name.clone(),
        members,
        system_order: VecDeque::new(),
        movies_watched: HashSet::new(),
        current_movies: HashSet::new(),
        ready_status: HashMap::new(),
        system_state: SystemState::AddingMovies,
        turn,
        date_created: now,
        date_modified: now,
    }
}

pub async fn acquire_db(db: &SqlitePool) -> Result<PoolConnection<Sqlite>> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;
    Ok(conn)
}

pub async fn db_get_user(db: &SqlitePool, username: &str) -> Result<DBUserStruct> {
    let mut conn = acquire_db(db).await?;
    log::info!("inside db_get_user");
    let db_user = query_as!(
        DBUser,
        r#"
            select *
            from users
            where username = $1
        "#,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    let user_struct = db_get_user_data(db_user)?;

    Ok(user_struct)
}

pub fn db_get_user_data(db_user: DBUser) -> Result<DBUserStruct> {
    let user_data: UserData = serde_json::from_str(&db_user.data)
        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
    let user_struct = DBUserStruct {
        username: db_user.username,
        user_data,
    };
    Ok(user_struct)
}

pub async fn db_insert_user(db: &SqlitePool, user_struct: DBUserStruct) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_user_data =
        serde_json::to_string(&user_struct.user_data).expect("serialization error");
    query!(
        r#"
            insert into users (username, data)
            values ($1, $2);
        "#,
        user_struct.username,
        serialized_user_data,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub async fn db_update_user(db: &SqlitePool, user_struct: DBUserStruct) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_user_data =
        serde_json::to_string(&user_struct.user_data).expect("serialization error");
    query!(
        r#"
            update users set data=$1 where username=$2
        "#,
        serialized_user_data,
        user_struct.username,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub async fn db_delete_user(db: &SqlitePool, username: &str) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    query!(
        r#"
                    delete from users
                    WHERE username = $1;
                    "#,
        username
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;
    Ok(())
}

pub async fn db_get_group(db: &SqlitePool, group_id: &str) -> Result<DBGroupStruct> {
    log::info!("inside db_get_group. group_id is: {:?}", &group_id);
    let mut conn = acquire_db(db).await?;
    let db_group = query_as!(
        DBGroup,
        r#"
            select *
            from groups
            where id = $1
        "#,
        group_id
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    let group_data = db_get_group_data(db_group)?;
    Ok(group_data)
}

pub async fn db_update_group(db: &SqlitePool, group_struct: &DBGroupStruct) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_group_data =
        serde_json::to_string(&group_struct.group_data).expect("serialization error");
    query!(
        r#"
            update groups set data=$1 where id=$2
        "#,
        serialized_group_data,
        group_struct.id,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub fn db_get_group_data(db_group: DBGroup) -> Result<DBGroupStruct> {
    let group_data: GroupData = serde_json::from_str(&db_group.data)
        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
    let group_struct = DBGroupStruct {
        id: db_group.id,
        group_data,
    };
    Ok(group_struct)
}

pub async fn db_insert_group(db: &SqlitePool, group_struct: DBGroupStruct) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_group_data =
        serde_json::to_string(&group_struct.group_data).expect("serialization error");
    query!(
        r#"
            insert into groups (id, data)
            values ($1, $2);
        "#,
        group_struct.id,
        serialized_group_data,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub async fn db_delete_group(db: &SqlitePool, group_id: &str) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    query!(
        r#"
                    delete from groups
                    WHERE id = $1;
                    "#,
        group_id
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;
    Ok(())
}

//Only used for tests currently
pub async fn db_update_password(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let now = sqlx::types::chrono::Utc::now().timestamp();

    let mut old_user_struct = db_get_user(db, &old_user.username).await?;

    match verify_pass(
        old_user.password.clone(),
        old_user_struct.user_data.salt,
        old_user_struct.user_data.hashed_password,
    )? {
        true => {
            let (new_hashed_password, new_salt) = hasher(&new_user.password).await?;
            old_user_struct.user_data.salt = new_salt;
            old_user_struct.user_data.hashed_password = new_hashed_password;
            old_user_struct.user_data.date_modified = now;
            db_update_user(db, old_user_struct).await?;
        }
        false => {
            custom(WarpRejections::AuthError(err_info!()));
        }
    }
    Ok(())
}

//Only used for tests currently
pub async fn db_update_username(
    db: &SqlitePool,
    old_user: &DBUserStruct,
    old_password: String,
) -> Result<()> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    let mut old_user_struct = db_get_user(db, &old_user.username).await?;
    match verify_pass(
        old_password,
        old_user_struct.user_data.salt.clone(),
        old_user_struct.user_data.hashed_password.clone(),
    )? {
        true => {
            old_user_struct.user_data.date_modified = now;
            db_update_user(db, old_user_struct).await?;
        }
        false => {
            custom(WarpRejections::AuthError(err_info!()));
        }
    }
    Ok(())
}
