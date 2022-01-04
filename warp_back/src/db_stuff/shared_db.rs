use crate::db_stuff::query_structs::User;
use crate::error_handling::{Result, SqlxError, WarpRejections};
use sqlx::pool::PoolConnection;
use sqlx::{query_as, Sqlite, SqlitePool};
use warp::reject::custom;

pub async fn acquire_db(db: &SqlitePool) -> Result<PoolConnection<Sqlite>> {
    let conn = db
        .acquire()
        .await
        .map_err(|_| custom(WarpRejections::SqlxRejection(SqlxError::DBConnectionError)))?;
    Ok(conn)
}

pub fn remove_one_from_list(list: String, remove: String) -> Result<String> {
    let list_vec = string_to_vec(list);
    let filtered = list_vec
        .iter()
        .filter(|item| **item != remove)
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let result_string = vec_to_string(filtered);
    Ok(result_string)
}

pub fn add_one_to_list(list: String, add: String) -> Result<String> {
    let mut list_vec = string_to_vec(list);
    list_vec.push(add);
    let result_string = vec_to_string(list_vec);
    Ok(result_string)
}

//Change to a match and use HashSet maybe?
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

pub async fn select_single_user(db: &SqlitePool, username: &str) -> Result<User> {
    let mut conn = acquire_db(db).await?;
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

pub async fn select_all_users(db: &SqlitePool) -> Result<Vec<User>> {
    let mut conn = acquire_db(db).await?;
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
