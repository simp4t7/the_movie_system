use crate::req;
use reqwest::header::HeaderValue;
use shared_stuff::imdb_structs::ImdbQuery;

use log::info;

pub async fn check_client_connection(
    input: ImdbQuery,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let (search_term, url) = req::build_url(input)?;
    info!("search term is: {}", &search_term);
    info!("full search is: {}", &url);
    let response = reqwest::get(url).await.map_err(|e| e.into());
    response
}

pub async fn status_and_headers(
    response: reqwest::Response,
    good_query: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    match good_query {
        true => {
            assert!(response.status().is_success());
            let headers = response.headers();
            assert_eq!(
                headers.get("content-type"),
                Some(&HeaderValue::from_static("application/javascript"))
            );
            let content_length: u32 = headers
                .get("content-length")
                .unwrap()
                .to_str()?
                .parse::<u32>()?;
            assert!(content_length > 0);
        }
        false => {
            info!("{:?}", response.status());
            assert!(response.status().is_client_error());
            let headers = response.headers();
            assert_eq!(
                headers.get("content-type"),
                Some(&HeaderValue::from_static("application/json"))
            );
            info!("Body: {:?}", response);
        }
    }

    Ok(())
}
