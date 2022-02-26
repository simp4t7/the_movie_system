use crate::{ACCESS_URL, REFRESH_URL};
use anyhow::{anyhow, Result};
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode, Response};
use shared_stuff::auth_structs::{Claims, TokenResponse};

pub async fn get_route_with_auth(url: &str) -> Result<Response> {
    let storage = LocalStorage::raw();
    let access_token = storage
        .get("access_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    let refresh_token = storage
        .get("refresh_token")
        .map_err(|e| anyhow!("storage error: {:?}", e))?;
    if let Some(token) = access_token {
        let request = make_get_request(url, &token);
        let resp = request.send().await?;
        match resp.status() {
            200 => Ok(resp),
            401 => {
                log::info!("access tokem 401");
                request_authorize_refresh(refresh_token.unwrap()).await?;
                let new_token = storage.get("access_token").expect("umm storage??").unwrap();
                let retry_request = make_get_request(url, &new_token);
                let retry_resp = retry_request.send().await?;
                Ok(retry_resp)
            }
            e => Err(anyhow!("weird status code: {:?}", e)),
        }
    } else if let Some(token) = refresh_token {
        request_authorize_refresh(token).await?;
        let new_token = storage.get("access_token").expect("umm storage??").unwrap();
        let retry_request = make_get_request(url, &new_token);
        let retry_resp = retry_request.send().await?;
        Ok(retry_resp)
    } else {
        Err(anyhow!("bad error uh oh"))
    }
}

pub async fn request_authorize_refresh(refresh_token: String) -> Result<TokenResponse> {
    let storage = LocalStorage::raw();
    let resp = Request::post(&REFRESH_URL)
        .mode(RequestMode::Cors)
        .header("authorization", &refresh_token)
        .send()
        .await?;
    let token_resp: TokenResponse = resp.json().await?;
    storage
        .set("access_token", &token_resp.access_token)
        .expect("storage error");

    //log::info!("{:?}", &access_token);
    Ok(token_resp)
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
            e => Err(anyhow!("Error body: {:?}", resp.text().await?)),
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

pub fn make_get_request(url: &str, token: &str) -> Request {
    let request = Request::get(url)
        .mode(RequestMode::Cors)
        .header("authorization", token);
    log::info!("get request: {:?}", request);
    request
}

pub fn make_post_request(url: &str, json_body: &str, token: &str) -> Request {
    Request::post(url)
        .mode(RequestMode::Cors)
        .header("authorization", token)
        .header("content-type", "application/json; charset=UTF-8")
        .body(json_body)
}
