use anyhow::Result;
use ctor::ctor;
use lazy_static::lazy_static;
use shared_stuff::omdb_structs::OmdbStruct;
use std::fs::File;
use std::io::Read;

const OMDB_URL: &str = "http://www.omdbapi.com/";
const API_KEY_PATH: &str = "../keys/omdb_api.txt";

lazy_static! {
    static ref API_KEY: String = {
        let mut file = File::open(API_KEY_PATH).expect("can't open file");
        let mut api_key = String::new();
        file.read_to_string(&mut api_key)
            .expect("problem reading file");
        api_key = api_key.trim().to_string();
        api_key
    };
}

#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}

async fn query_builder(id: &str) -> Result<OmdbStruct> {
    let query_url = format!("{}?i={}&plot=full&apikey={}", OMDB_URL, id, *API_KEY);
    log::info!("query is: {:?}", &query_url);

    let response = reqwest::get(&query_url).await?;
    let body_text: OmdbStruct = response.json().await?;
    log::info!("body_text is: {:?}", &body_text);
    //let omdb_struct: OmdbStruct = serde_json::from_str(&body_text)?;
    //log::info!("omdb_struct is: {:?}", &omdb_struct);
    Ok(body_text)
}

mod tests {

    #[tokio::test]
    async fn first_test() -> Result<()> {
        query_builder("tt1160419").await?;
        Ok(())
    }
}
