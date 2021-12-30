use crate::auth::verify_pass;

use crate::error_handling::{AuthError, Result, SqlxError, WarpRejections};

use shared_stuff::groups_stuff::AddUser;

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

#[derive(Debug)]
pub struct GroupsStruct {
    groups: Option<String>,
}
#[derive(Debug)]
pub struct GroupIdStruct {
    id: String,
}

#[derive(Debug)]
pub struct GroupNameStruct {
    name: String,
}
#[derive(Debug)]
pub struct QueryGroupMembers {
    members: String,
}

pub async fn get_group_members(db: &SqlitePool, group_id: &str) -> Result<String> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;

    let members_struct = query_as!(
        QueryGroupMembers,
        r#"
                    select members from groups
                    WHERE id = $1;
                    "#,
        group_id,
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;

    Ok(members_struct.members)
}

pub async fn get_user_groups(db: &SqlitePool, user: &str) -> Result<String> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;

    let groups = query_as!(
        GroupsStruct,
        r#"
                    select groups from users
                    WHERE username = $1;
                    "#,
        user,
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;

    if let Some(user_groups) = groups.groups {
        Ok(user_groups)
    } else {
        Ok(String::from(""))
    }
}

pub async fn get_group_names(db: &SqlitePool, group_ids: Vec<String>) -> Result<Vec<String>> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let mut res_vec = vec![];
    for id in group_ids {
        let group_name = query_as!(
            GroupNameStruct,
            r#"
                    select name from groups
                    WHERE id = $1;
                    "#,
            id,
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
        res_vec.push(group_name.name);
    }
    Ok(res_vec)
}

pub async fn get_group_id(db: &SqlitePool, group_name: &str, username: &str) -> Result<String> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let username = format!("%{}%", username);
    let group_id = query_as!(
        GroupIdStruct,
        r#"
                    select id from groups 
                    WHERE name=$1 AND members LIKE $2
                    "#,
        group_name,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
    Ok(group_id.id)
}

pub async fn update_group_members(db: &SqlitePool, group_id: &str, members: &str) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let _updated_memebers = query!(
        r#"
                    update groups
                    set members = $1
                    WHERE id = $2;
                    "#,
        members,
        group_id
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
    Ok(())
}

pub async fn update_user_groups(db: &SqlitePool, user: &str, groups: &str) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let _updated_groups = query!(
        r#"
                    update users
                    set groups = $1
                    WHERE username = $2;
                    "#,
        groups,
        user
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
    Ok(())
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

pub async fn delete_group(db: &SqlitePool, group_id: &str) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    query!(
        r#"
                    delete from groups
                    WHERE id = $1;
                    "#,
        group_id
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
    Ok(())
}

fn remove_one_from_list(list: String, remove: String) -> Result<String> {
    let list_vec = string_to_vec(list);
    let filtered = list_vec
        .iter()
        .filter(|item| **item != remove)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let result_string = vec_to_string(filtered);
    Ok(result_string)
}

fn add_one_to_list(list: String, add: String) -> Result<String> {
    let mut list_vec = string_to_vec(list);
    list_vec.push(add);
    let result_string = vec_to_string(list_vec);
    Ok(result_string)
}

//Change to a match
pub fn string_to_vec(input: String) -> Vec<String> {
    log::info!("input is: {:?}", &input);
    if input.contains(',') {
        let res_vec = input
            .split(',')
            .map(|item| item.trim().to_string())
            .collect::<Vec<String>>();
        res_vec
    } else if input == *"" {
        let res_vec = vec![];
        res_vec
    } else {
        let res_vec = vec![input];
        res_vec
    }
}

fn vec_to_string(mut input: Vec<String>) -> String {
    if input.len() > 1 {
        let start = input.remove(0);
        input
            .iter()
            .fold(start, |acc, item| format!("{} , {}", acc, item))
    } else if input.len() == 1 {
        input.remove(0)
    } else {
        String::from("")
    }
}

pub async fn db_add_user(db: &SqlitePool, user_struct: &AddUser) -> Result<()> {
    let _conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;

    let username = user_struct.username.clone();
    let add_user = user_struct.add_user.clone();
    let group_name = user_struct.group_name.clone();

    let group_id = get_group_id(db, &group_name, &username).await?;

    let group_members = get_group_members(db, &group_id).await?;

    log::info!("group members: {:?}", &group_members);
    let added = add_one_to_list(group_members.clone(), add_user.clone())?;
    log::info!("added: {:?}", &added);

    match select_single_user(db, &add_user).await {
        Ok(_) => {
            log::info!("in here");
            update_group_members(db, &group_id, &added).await?;
            let groups = get_user_groups(db, &add_user).await?;
            let added_groups = add_one_to_list(groups, group_id.clone())?;
            update_user_groups(db, &add_user, &added_groups).await?;
        }
        Err(e) => return Err(e),
    }

    Ok(())
}

// NOTHING WRONG WITH 200 LINE FUNCTIONS...
pub async fn leave_user_group(db: &SqlitePool, username: &str, group_name: String) -> Result<()> {
    log::info!(
        "username is: {:?}, group_id is: {:?}",
        &username,
        &group_name
    );
    let _conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let groups = get_user_groups(db, username).await?;

    let group_id = get_group_id(db, &group_name, username).await?;

    let _updated_groups = remove_one_from_list(groups, group_id.clone())?;

    update_user_group(db, username, &group_id).await?;

    let members = get_group_members(db, &group_id).await?;

    let members_string = remove_one_from_list(members, username.to_string())?;

    if members_string == *"" {
        delete_group(db, &group_id).await?;
    } else {
        log::info!("gotta update");
        update_group_members(db, &group_id, &members_string).await?;
    }

    log::info!("made it to the end...?");

    Ok(())
}

pub async fn update_user_group(db: &SqlitePool, username: &str, group_id: &str) -> Result<()> {
    let mut conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    let query = query_as!(
        GroupsStruct,
        r#"
                    select groups from users 
                    WHERE username=$1
                    "#,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::CreateGroupError)))?;

    log::info!("{:?}", &query);

    let mut new_entry = String::from("");
    match query.groups {
        Some(entry) if entry != new_entry => {
            new_entry = format!("{} , {}", &entry, &group_id);
        }
        _ => {
            new_entry = group_id.to_string();
        }
    }

    query!(
        r#"

                    UPDATE users 
                    set groups=$1
                    WHERE username=$2
                    "#,
        new_entry,
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

    query!(
        r#"

        INSERT INTO groups ( id, name, members, movies_watched, current_movies, turn, date_created, date_modified  )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)

        "#,
        uuid,
        group_form.group_name,
        group_form.username,
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
