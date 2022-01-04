use crate::{ACCESS_URL, REFRESH_URL};
use anyhow::anyhow;
use anyhow::Result;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::Claims;
use shared_stuff::DoubleTokenResponse;
use shared_stuff::SingleTokenResponse;
use shared_stuff::{ImageData, ImdbQuery, MovieDisplay, UserInfo};

pub async fn authorize_refresh(refresh_token: String) -> Result<SingleTokenResponse> {
    let storage = LocalStorage::raw();
    let resp = Request::post(&REFRESH_URL)
        .mode(RequestMode::Cors)
        .header("authorization", &refresh_token)
        .send()
        .await?;
    let single_token: SingleTokenResponse = resp.json().await?;
    storage
        .set("access_token", &single_token.access_token)
        .expect("storage error");

    //log::info!("{:?}", &access_token);
    Ok(single_token)
}
pub async fn authorize_access(access_token: String) -> Result<Claims> {
    let resp = Request::post(&ACCESS_URL)
        .mode(RequestMode::Cors)
        .header("authorization", &access_token)
        .send()
        .await?;
    let claims: Claims = resp.json().await?;

    log::info!("{:?}", &claims);
    Ok(claims)
}

pub async fn auth_flow() -> Result<Claims> {
    let storage = LocalStorage::raw();
    let access_token = storage
        .get("access_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    let refresh_token = storage
        .get("refresh_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    if let Some(token) = access_token {
        let resp = Request::post(&ACCESS_URL)
            .mode(RequestMode::Cors)
            .header("authorization", &token)
            .send()
            .await?;
        match resp.status() {
            200 => {
                let claims: Claims = resp.json().await?;
                log::info!("{:?}", &claims);
                Ok(claims)
            }
            401 => {
                authorize_refresh(refresh_token.unwrap()).await?;
                let new_token = storage.get("access_token").expect("umm storage??").unwrap();
                let claims = authorize_access(new_token).await?;
                Ok(claims)
            }
            e => Err(anyhow!("weird status code: {:?}", e)),
        }
    } else if let Some(token) = refresh_token {
        authorize_refresh(token).await?;
        let new_token = storage.get("access_token").expect("umm storage??").unwrap();
        let claims = authorize_access(new_token).await?;
        Ok(claims)
    } else {
        Err(anyhow!("bad error uh oh"))
    }
}

//Need something if there's no picture or poster...
//There's a lot more processing to be done for different size images, but
//mostly works now and whatever.
pub fn image_processing(image: Option<&ImageData>) -> String {
    if let Some(image) = image {
        let mut image_url = image.url.to_owned();
        assert!(&image_url[image_url.len() - 4..] == ".jpg");
        image_url.truncate(image_url.len() - 4);
        image_url.push_str("._V1_QL75_UY74_CR30,0,50,74_.jpg");
        image_url
    } else {
        "need to get a decent no pic available pic".to_string()
    }
}
