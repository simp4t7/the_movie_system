use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::ImageData;
use shared_stuff::MovieDisplay;
use shared_stuff::UserInfo;

pub async fn get_search_results(
    url: &str,
    body: String,
) -> Result<Vec<MovieDisplay>, Box<dyn std::error::Error>> {
    let resp = Request::post(url)
        //.mode(RequestMode::Cors)
        .body(body)
        .send()
        .await?
        .json()
        .await?;
    Ok(resp)
}

pub async fn send_user_info(url: &str, body: UserInfo) -> Result<(), Box<dyn std::error::Error>> {
    let userinfo = serde_json::to_string(&body)?;
    let resp = Request::post(url)
        .header("Content-Type", "application/json")
        .mode(RequestMode::NoCors)
        .body(userinfo)
        .send()
        .await?;
    Ok(())
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
