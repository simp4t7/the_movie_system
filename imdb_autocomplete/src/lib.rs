use shared_stuff::imdb_structs::ImdbQuery;
use shared_stuff::shared_structs::MovieDisplay;
use std::io::ErrorKind;

pub mod req;
pub mod res;
pub mod test_functions;

use log::{info, trace};

pub async fn autocomplete_func(
    input: ImdbQuery,
) -> Result<Vec<MovieDisplay>, Box<dyn std::error::Error>> {
    let (search_term, url) = req::build_url(input)?;
    log::info!("{:?}", &url);

    let response = reqwest::get(url).await?;
    log::info!("{:?}", &response);
    match response.status().is_success() {
        true => {
            let body_string = response.text().await?;
            trace!("{:?}", body_string);
            let res = res::make_movie_display(body_string, search_term);
            info!("{:?}", &res);
            res
        }
        false => {
            // captain wants some bad request error
            let custom_error = std::io::Error::new(ErrorKind::Other, "oh no! bad request");
            Err(custom_error.into())
        }
    }
}
