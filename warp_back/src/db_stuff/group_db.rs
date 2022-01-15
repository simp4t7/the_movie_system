use crate::db_stuff::query_structs::{
    GroupCurrentMovies, GroupId, GroupMembers, GroupName, UserGroups,
};
use crate::db_stuff::shared_db::{
    acquire_db, add_one_to_list, remove_one_from_list, select_single_user,
};
use crate::error_handling::{AuthError, Result, SqlxError, WarpRejections};
use shared_stuff::groups_stuff::{AddUser, GroupForm, GroupMoviesForm};
use shared_stuff::{MovieDisplay, YewMovieDisplay};
use sqlx::SqlitePool;
use sqlx::{query, query_as};
use std::collections::HashSet;
use uuid::Uuid;
use warp::reject::custom;

pub async fn get_group_members(db: &SqlitePool, group_id: &str) -> Result<String> {
    let mut conn = acquire_db(db).await?;

    let members_struct = query_as!(
        GroupMembers,
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
    let mut conn = acquire_db(db).await?;

    let groups = query_as!(
        UserGroups,
        r#"
                    select groups from users
                    WHERE username = $1;
                    "#,
        user,
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::SelectGroupsError)))?;

    if let Some(user_groups) = groups.groups {
        Ok(user_groups)
    } else {
        Ok(String::from(""))
    }
}

pub async fn get_all_group_names(db: &SqlitePool, group_ids: Vec<String>) -> Result<Vec<String>> {
    let mut conn = acquire_db(db).await?;
    let mut res_vec = vec![];
    for id in group_ids {
        let group_name = query_as!(
            GroupName,
            r#"
                    select name from groups
                    WHERE id = $1;
                    "#,
            id,
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::GroupError)))?;
        res_vec.push(group_name.name);
    }
    Ok(res_vec)
}

pub async fn get_group_id(db: &SqlitePool, group_name: &str, username: &str) -> Result<String> {
    let mut conn = acquire_db(db).await?;
    let username = format!("%{}%", username);
    let group_id = query_as!(
        GroupId,
        r#"
                    select id from groups 
                    WHERE name=$1 AND members LIKE $2
                    "#,
        group_name,
        username
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::GroupError)))?;
    Ok(group_id.id)
}

pub async fn update_group_members(db: &SqlitePool, group_id: &str, members: &str) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    query!(
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
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::GroupError)))?;
    Ok(())
}

pub async fn db_save_group_movies(db: &SqlitePool, group_info: &GroupMoviesForm) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let username = group_info.username.clone();
    let group_name = group_info.group_name.clone();
    log::info!("current movies: {:?}", &group_info.current_movies);

    let group_id = get_group_id(db, &group_name, &username).await?;
    log::info!("group_id is: {:?}", &group_id);
    let mut serialized_movies: HashSet<YewMovieDisplay> = group_info.current_movies.clone();

    let json_movies = serde_json::to_string(&serialized_movies)
        .map_err(|_| custom(WarpRejections::SerializationError))?;

    query!(
        r#"
                    update groups
                    set current_movies = $1
                    WHERE id = $2;
                    "#,
        json_movies,
        group_id,
    )
    .execute(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::SaveMoviesError)))?;
    log::info!("got past the stuff?");

    Ok(())
}

pub async fn db_get_group_movies(
    db: &SqlitePool,
    group_form: &GroupForm,
) -> Result<HashSet<YewMovieDisplay>> {
    let mut conn = acquire_db(db).await?;
    let username = group_form.username.clone();
    let group_name = group_form.group_name.clone();

    let group_id = get_group_id(db, &group_name, &username).await?;
    let current_movies = query_as!(
        GroupCurrentMovies,
        r#"
                    select current_movies from groups
                    WHERE id = $1;
                    "#,
        group_id,
    )
    .fetch_one(&mut conn)
    .await
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;

    if let Some(current) = current_movies.current_movies {
        let serialized: HashSet<YewMovieDisplay> = serde_json::from_str(&current)
            .map_err(|_| custom(WarpRejections::SerializationError))?;
        return Ok(serialized);
    } else {
        return Ok(HashSet::new());
    }
}

pub async fn update_user_groups(db: &SqlitePool, user: &str, groups: &str) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    query!(
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

pub async fn db_add_user_to_group(db: &SqlitePool, user_struct: &AddUser) -> Result<()> {
    let conn = acquire_db(db).await?;

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

pub async fn delete_group(db: &SqlitePool, group_id: &str) -> Result<()> {
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
    .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DeleteGroupError)))?;
    Ok(())
}

pub async fn leave_user_group(db: &SqlitePool, group_form: &GroupForm) -> Result<()> {
    log::info!("group_form is: {:?}", &group_form,);
    let username = &group_form.username;
    let group_name = &group_form.group_name;
    let groups = get_user_groups(db, username).await?;

    let group_id = get_group_id(db, group_name, username).await?;

    let updated_groups = remove_one_from_list(groups, group_id.clone())?;

    update_user_groups(db, username, &updated_groups).await?;

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

pub async fn add_to_user_groups(db: &SqlitePool, username: &str, group_id: &str) -> Result<()> {
    let mut conn = acquire_db(db).await?;
    let query = query_as!(
        UserGroups,
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

    update_user_groups(db, username, &new_entry).await?;

    Ok(())
}

pub async fn create_new_group(db: &SqlitePool, group_form: &GroupForm) -> Result<String> {
    let mut conn = acquire_db(db).await?;

    let now = sqlx::types::chrono::Utc::now();
    let uuid = Uuid::new_v4().to_string();

    query!(
        r#"

        INSERT INTO groups ( id, name, members, movies_watched, 
        current_movies, turn, date_created, date_modified  )
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
