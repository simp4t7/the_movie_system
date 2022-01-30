use crate::auth_requests::get_route_with_auth;
use crate::GET_GROUP_DATA_URL;
use anyhow::Result;
use shared_stuff::db_structs::DBGroupStruct;

pub async fn request_get_group_data(group_id: String) -> Result<DBGroupStruct> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    log::info!("request_get_all_group_movies resp: {:?}", &resp);
    let group_struct: DBGroupStruct = resp.json().await?;
    log::info!(
        "request_get_all_group_movies group_data: {:?}",
        &group_struct
    );
    Ok(group_struct)
}
