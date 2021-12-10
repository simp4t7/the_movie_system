use anyhow::anyhow;
use anyhow::Result;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::{ImageData, ImdbQuery, MovieDisplay, UserInfo};

pub async fn get_search_results(url: &str, body: ImdbQuery) -> Result<Vec<MovieDisplay>> {
    let imdbquery = serde_json::to_string(&body)?;
    let resp = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(imdbquery)
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

pub async fn register_request(url: &str, body: UserInfo) -> Result<()> {
    let userinfo = serde_json::to_string(&body)?;
    log::info!("{:?}", &userinfo);
    let resp = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(userinfo)
        .send()
        .await?;
    //.text()
    //.await?;
    log::info!("{:?}", &resp);
    Ok(())
}

pub async fn login_request(url: &str, body: UserInfo) -> Result<String> {
    let userinfo = serde_json::to_string(&body)?;
    log::info!("{:?}", &userinfo);
    let resp = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(userinfo)
        .send()
        .await?;

    let auth_value = resp
        .headers()
        .get("authorization")
        .map_err(|e| anyhow!("header error: {:?}", e))?;

    if let Some(token) = auth_value {
        return Ok(token);
    } else {
        return Err(anyhow!("token error: {:?}", auth_value));
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
