use crate::auth::{hasher, verify_pass};

use crate::err_info;
use crate::error_handling::{Result, WarpRejections};
use shared_stuff::db_structs::{DBGroup, DBUser, GroupData, UserData};
use shared_stuff::groups_stuff::{AddUser, GroupForm, GroupInfo, GroupMoviesForm};
use shared_stuff::UserInfo;
use shared_stuff::YewMovieDisplay;
use sqlx::pool::PoolConnection;
use sqlx::Sqlite;
use sqlx::{query, query_as, SqlitePool};
use std::collections::HashSet;
use uuid::Uuid;
use warp::reject::custom;

pub async fn create_user_data(input: UserInfo) -> Result<UserData> {
    let id = Uuid::new_v4();
    let (hashed_password, salt) = hasher(&input.password).await?;
    let groups = HashSet::new();
    let now = chrono::Utc::now().timestamp();
    Ok(UserData {
        id,
        hashed_password,
        salt,
        groups,
        date_created: now,
        date_modified: now,
    })
}

pub fn create_group_data(input: GroupForm) -> GroupData {
    let mut members_hash = HashSet::new();
    members_hash.insert(input.username);
    let now = chrono::Utc::now().timestamp();
    let turn = String::from("");
    GroupData {
        group_name: input.group_name,
        members: members_hash,
        movies_watched: HashSet::new(),
        current_movies: HashSet::new(),
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

pub async fn db_get_user(db: &SqlitePool, username: &str) -> Result<(String, UserData)> {
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

    let user_data = db_get_user_data(db_user)?;

    Ok(user_data)
}

pub fn db_get_user_data(db_user: DBUser) -> Result<(String, UserData)> {
    log::info!("inside db_get_user_data");
    log::info!("DBUser is: {:?}", &db_user);
    log::info!("DBUser data is: {:?}", &db_user.data);
    let user_data: UserData = serde_json::from_str(&db_user.data)
        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
    Ok((db_user.username, user_data))
}

pub async fn db_insert_user(db: &SqlitePool, username: &str, user_data: UserData) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_user_data = serde_json::to_string(&user_data).expect("serialization error");
    query!(
        r#"
            insert into users (username, data)
            values ($1, $2);
        "#,
        username,
        serialized_user_data,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub async fn db_update_user(db: &SqlitePool, user_data: (String, UserData)) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_user_data = serde_json::to_string(&user_data.1).expect("serialization error");
    query!(
        r#"
            update users set data=$1 where username=$2
        "#,
        serialized_user_data,
        user_data.0,
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

pub async fn verify_group_member(group_id: String, username: String, db: &SqlitePool) -> Result<GroupData> {
    let group_data = db_get_group1(db, &group_id).await?;

    let members = &group_data.members;
    if members.contains(&username) {
        Ok(group_data)
    } else {
        Err(custom(WarpRejections::UserNotInGroup(err_info!())))
    }

}

pub async fn db_get_group1(db: &SqlitePool, group_id: &str) -> Result<GroupData> {
    log::info!("inside db_get_group1. group_id is: {:?}", &group_id);
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
    .await;

    match db_group {
        Ok(db_group) => {
            let group_data: GroupData = serde_json::from_str(&db_group.data)
                .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
            Ok(group_data)
        },
        Err(_) => {
            log::info!("Cannot find group_id {} in db", &group_id);
            Err(custom(WarpRejections::GroupNotExist(err_info!())))?
        }
    }

}

pub async fn db_get_group(db: &SqlitePool, group_id: &str) -> Result<(String, GroupData)> {
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

pub fn db_get_group_data(db_group: DBGroup) -> Result<(String, GroupData)> {
    log::info!("DBGroup is: {:?}", &db_group);
    let group_data: GroupData = serde_json::from_str(&db_group.data)
        .map_err(|_| custom(WarpRejections::SerializationError(err_info!())))?;
    log::info!("group_data is: {:?}", &group_data);
    Ok((db_group.id, group_data))
}

pub async fn db_update_group(db: &SqlitePool, group_data: (String, GroupData)) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_group_data = serde_json::to_string(&group_data.1).expect("serialization error");
    query!(
        r#"
            update groups set data=$1 where id=$2
        "#,
        serialized_group_data,
        group_data.0,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxError(err_info!())))?;

    Ok(())
}

pub async fn db_insert_group(db: &SqlitePool, group_id: &str, group_data: GroupData) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let serialized_group_data = serde_json::to_string(&group_data).expect("serialization error");
    query!(
        r#"
            insert into groups (id, data)
            values ($1, $2);
        "#,
        group_id,
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

pub async fn db_get_group_members(db: &SqlitePool, group_id: &str) -> Result<HashSet<String>> {
    log::info!("inside db_get_group_members");
    let db_group_data = db_get_group1(db, group_id).await?;
    let members = db_group_data.members;
    log::info!("members are: {:?}", &members);
    Ok(members)
}

pub async fn db_get_user_groups(db: &SqlitePool, username: &str) -> Result<HashSet<GroupInfo>> {
    let db_user_data = db_get_user(db, username).await?;
    let user_groups = db_user_data.1.groups;
    Ok(user_groups)
}

pub async fn db_get_all_group_names(db: &SqlitePool, username: &str) -> Result<HashSet<GroupInfo>> {
    let db_user_data = db_get_user(db, username).await?;
    let user_groups = db_user_data.1.groups;
    Ok(user_groups)
}

pub async fn db_get_group_id(db: &SqlitePool, group_name: &str, username: &str) -> Result<String> {
    log::info!(
        "db_get_group_id group_name: {:?}, username: {:?}",
        &group_name,
        &username
    );
    let db_user_data = db_get_user(db, username).await?;
    let user_groups = db_user_data.1.groups;
    let option_group_info = user_groups.iter().find(|group_info| {
        log::info!("name: {:?}, uuid: {:?}", &group_info.name, &group_info.uuid);
        group_info.name.as_str() == group_name
    });
    log::info!("option_id: {:?}", &option_group_info);
    if let Some(info) = option_group_info {
        Ok(info.uuid.clone())
    } else {
        Err(custom(WarpRejections::SqlxError(err_info!())))
    }
}

pub async fn db_add_user_to_group(db: &SqlitePool, add_user: &AddUser) -> Result<()> {
    let group_id = db_get_group_id(db, &add_user.group_name, &add_user.username).await?;
    let mut db_group_data = db_get_group(db, &group_id).await?;
    let mut db_group_members = db_group_data.1.members;
    db_group_members.insert(add_user.new_member.to_string());
    log::info!("db_group_members: {:?}", &db_group_members);
    db_group_data.1.members = db_group_members;
    log::info!("db_group_data: {:?}", &db_group_data);
    db_update_group(db, db_group_data).await?;
    let mut user_info = db_get_user(db, &add_user.new_member).await?;
    let group_uuid =
        Uuid::parse_str(&group_id).map_err(|_e| custom(WarpRejections::UuidError(err_info!())))?;
    let group_info = GroupInfo {
        uuid: group_uuid.to_string(),
        name: add_user.group_name.clone(),
    };
    user_info.1.groups.insert(group_info);
    db_update_user(db, user_info).await?;
    Ok(())
}

pub async fn db_save_group_movies(db: &SqlitePool, group_info: &GroupMoviesForm) -> Result<()> {
    let username = &group_info.username;
    let group_name = &group_info.group_name;
    let group_id = db_get_group_id(db, group_name, username).await?;
    let current_movies: HashSet<YewMovieDisplay> = group_info.current_movies.clone();
    let mut group_data = db_get_group(db, &group_id).await?;
    log::info!("current_movies: {:?}", &current_movies);
    group_data.1.current_movies = current_movies;
    db_update_group(db, group_data).await?;

    Ok(())
}

pub async fn db_get_group_movies(
    db: &SqlitePool,
    group_form: &GroupForm,
) -> Result<HashSet<YewMovieDisplay>> {
    let username = &group_form.username;
    let group_name = &group_form.group_name;
    let group_id = db_get_group_id(db, group_name, username).await?;
    let group_data: GroupData = db_get_group(db, &group_id).await?.1;
    Ok(group_data.current_movies)
}

pub async fn db_add_group_to_user(
    db: &SqlitePool,
    mut user_data: (String, UserData),
    group: GroupInfo,
) -> Result<()> {
    let mut new_groups = user_data.1.groups;
    new_groups.insert(group);
    user_data.1.groups = new_groups;
    db_update_user(db, user_data).await?;
    Ok(())
}

pub async fn db_group_add_new_user(db: &SqlitePool, user_struct: &AddUser) -> Result<()> {
    let username = &user_struct.username;
    let new_member = &user_struct.new_member;
    let group_name = &user_struct.group_name;
    let group_id = db_get_group_id(db, group_name, username).await?;
    let group_uuid =
        Uuid::parse_str(&group_id).map_err(|_e| custom(WarpRejections::UuidError(err_info!())))?;

    match db_get_user(db, new_member).await {
        Ok(user_data) => {
            log::info!("in here");
            let group_info = GroupInfo {
                uuid: group_uuid.to_string(),
                name: group_name.clone(),
            };
            db_add_group_to_user(db, user_data.clone(), group_info).await?;
            db_add_user_to_group(db, user_struct).await?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

pub async fn db_user_leave_group(db: &SqlitePool, group_form: &GroupForm) -> Result<()> {
    log::info!("group_form is: {:?}", &group_form,);
    let username = &group_form.username;
    let group_name = &group_form.group_name;
    let groups = db_get_user_groups(db, username).await?;
    log::info!("groups are: {:?}", &groups);

    let group_id = db_get_group_id(db, group_name, username).await?;
    log::info!("group_id is: {:?}", &group_id);
    let mut group_data = db_get_group(db, &group_id).await?;
    log::info!("group_data is: {:?}", &group_data);
    let mut user_data = db_get_user(db, username).await?;
    log::info!("user_data is: {:?}", &user_data);
    let user_groups = db_get_user_groups(db, username)
        .await?
        .iter()
        .filter(|group_info| !group_info.name.eq(group_name))
        .cloned()
        .collect::<HashSet<GroupInfo>>();
    log::info!("user_groups is: {:?}", &user_groups);
    user_data.1.groups = user_groups;

    db_update_user(db, user_data).await?;

    let group_members = db_get_group_members(db, &group_id)
        .await?
        .iter()
        .filter(|name| *name != username)
        .map(|name| name.to_owned())
        .collect::<HashSet<String>>();
    log::info!("group_members is: {:?}", &group_members);
    group_data.1.members = group_members.clone();

    match group_members.is_empty() {
        true => {
            log::info!("inside true");
            db_delete_group(db, &group_id).await?
        }
        false => {
            log::info!("inside false");
            db_update_group(db, group_data).await?
        }
    }

    Ok(())
}

pub async fn db_update_password(
    db: &SqlitePool,
    old_user: &UserInfo,
    new_user: &UserInfo,
) -> Result<()> {
    let now = sqlx::types::chrono::Utc::now().timestamp();

    match db_get_user(db, &old_user.username).await {
        Ok(mut old_user_info) => match verify_pass(
            old_user.password.clone(),
            old_user_info.1.salt,
            old_user_info.1.hashed_password,
        )? {
            true => {
                let (new_hashed_password, new_salt) = hasher(&new_user.password).await?;
                old_user_info.1.salt = new_salt;
                old_user_info.1.hashed_password = new_hashed_password;
                old_user_info.1.date_modified = now;
                db_update_user(db, old_user_info).await?;
            }
            false => {
                custom(WarpRejections::AuthError(err_info!()));
            }
        },
        Err(_) => return Err(custom(WarpRejections::SqlxError(err_info!()))),
    }
    Ok(())
}

pub async fn db_update_username(db: &SqlitePool, old_user: &UserInfo) -> Result<()> {
    let now = sqlx::types::chrono::Utc::now().timestamp();
    match db_get_user(db, &old_user.username).await {
        Ok(mut user_info) => match verify_pass(
            old_user.password.clone(),
            user_info.1.salt.clone(),
            user_info.1.hashed_password.clone(),
        )? {
            true => {
                user_info.1.date_modified = now;
                db_update_user(db, user_info).await?;
            }
            false => {
                custom(WarpRejections::AuthError(err_info!()));
            }
        },
        Err(_) => return Err(custom(WarpRejections::SqlxError(err_info!()))),
    }
    Ok(())
}
