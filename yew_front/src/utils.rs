use crate::{ACCESS_URL, REFRESH_URL};
use anyhow::anyhow;
use anyhow::Result;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use reqwasm::http::Response;
use shared_stuff::Claims;

use shared_stuff::ImageData;
use shared_stuff::SingleTokenResponse;

pub async fn request_authorize_refresh(refresh_token: String) -> Result<SingleTokenResponse> {
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
pub async fn request_authorize_access(access_token: String) -> Result<Claims> {
    let resp = Request::post(&ACCESS_URL)
        .mode(RequestMode::Cors)
        .header("authorization", &access_token)
        .send()
        .await?;
    let claims: Claims = resp.json().await?;

    log::info!("{:?}", &claims);
    Ok(claims)
}

pub async fn request_auth_flow() -> Result<Claims> {
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
        log::info!("resp auth_flow: {:?}", &resp);

        match resp.status() {
            200 => {
                let claims: Claims = resp.json().await?;
                log::info!("{:?}", &claims);
                Ok(claims)
            }
            401 => {
                request_authorize_refresh(refresh_token.unwrap()).await?;
                let new_token = storage.get("access_token").expect("umm storage??").unwrap();
                let claims = request_authorize_access(new_token).await?;
                Ok(claims)
            }
            e => Err(anyhow!("weird status code: {:?}", e)),
        }
    } else if let Some(token) = refresh_token {
        request_authorize_refresh(token).await?;
        let new_token = storage.get("access_token").expect("umm storage??").unwrap();
        let claims = request_authorize_access(new_token).await?;
        Ok(claims)
    } else {
        Err(anyhow!("bad error uh oh"))
    }
}

/// Post Request with tokens included in the header's authorization field.
/// Flow:
///     send the request to `url` with access_token
///         200 => return response
///         401 => send request to `REFRESH_URL` to apply for a new access_token and try again
///         error => return error
pub async fn post_route_with_auth(url: &str, json_body: String) -> Result<Response> {
    let storage = LocalStorage::raw();
    let access_token = storage
        .get("access_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    let refresh_token = storage
        .get("refresh_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    if let Some(token) = access_token {
        let request = make_post_request(url, &json_body, &token);
        let resp = request.send().await?;
        match resp.status() {
            200 => Ok(resp),
            401 => {
                log::info!("access tokem 401");
                request_authorize_refresh(refresh_token.unwrap()).await?;
                let new_token = storage.get("access_token").expect("umm storage??").unwrap();
                let retry_request = make_post_request(url, &json_body, &new_token);
                let retry_resp = retry_request.send().await?;
                Ok(retry_resp)
            }
            e => Err(anyhow!("weird status code: {:?}", e)),
        }
    } else if let Some(token) = refresh_token {
        request_authorize_refresh(token).await?;
        let new_token = storage.get("access_token").expect("umm storage??").unwrap();
        let retry_request = make_post_request(url, &json_body, &new_token);
        let retry_resp = retry_request.send().await?;
        Ok(retry_resp)
    } else {
        Err(anyhow!("bad error uh oh"))
    }
}

pub fn make_post_request(url: &str, json_body: &str, token: &str) -> Request {
    Request::post(url)
        .mode(RequestMode::Cors)
        .header("authorization", token)
        .header("content-type", "application/json; charset=UTF-8")
        .body(json_body)
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
