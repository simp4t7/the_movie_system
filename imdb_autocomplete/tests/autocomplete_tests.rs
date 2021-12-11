use ctor::ctor;
use imdb_autocomplete::autocomplete_func;
use imdb_autocomplete::test_functions::check_client_connection;
use imdb_autocomplete::test_functions::status_and_headers;

use log::{debug, error, info, log, trace, warn};

//use shared_stuff::utils::load_logger;
use shared_stuff::MovieDisplay;
use std::sync::Once;

#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}

//static INIT: Once = Once::new();

//fn logger_init() {
//INIT.call_once(|| {
//load_logger().unwrap();
//})
//}

// GET A GOOD LIST OF QUERIES TO TEST
const LONGEST_MOVIE_TITLE: &str = "Night of the Day of the Dawn of the Son of the Bride of the Return of the Revenge of the Terror of the Attack of the Evil, Mutant, Hellbound, Flesh-Eating Subhumanoid Zombified Living Dead, Part 3";
const SHORTEST_MOVIE_TITLE: &str = "B";
const NUMBERS: &str = "11:14";
const JUST_DOLLAR_SIGN: &str = "$";
const STARS: &str = "****";
const ROMEO: &str = "Romeo + Juliet";
const TERRIBLE_QUERY: &str = "@#*(SA+_]]\\'.)";
const DUNE: &str = "Dune";
const GIBBERISH: &str = "nfjdakerejerkrj";

#[tokio::test]
async fn check_connection_1() -> Result<(), Box<dyn std::error::Error>> {
    let res_1 = check_client_connection(LONGEST_MOVIE_TITLE.into()).await?;
    status_and_headers(res_1, true).await?;
    Ok(())
}
//THIS TEST COULD BE BETTER, FAILS BEFORE CONNECTION
#[tokio::test]
async fn check_connection_2() {
    let res_2 = check_client_connection(JUST_DOLLAR_SIGN.into()).await;
    assert!(res_2.is_err());
    log::info!("2");
}
#[tokio::test]
async fn check_connection_3() -> Result<(), Box<dyn std::error::Error>> {
    let res_3 = check_client_connection(TERRIBLE_QUERY.into()).await?;
    status_and_headers(res_3, false).await?;
    log::info!("3");
    Ok(())
}
#[tokio::test]
async fn check_connection_4() -> Result<(), Box<dyn std::error::Error>> {
    let res_4 = check_client_connection(NUMBERS.into()).await?;
    status_and_headers(res_4, true).await?;
    log::info!("4");
    Ok(())
}
#[tokio::test]
async fn check_connection_5() -> Result<(), Box<dyn std::error::Error>> {
    let res_5 = check_client_connection(SHORTEST_MOVIE_TITLE.into()).await?;
    status_and_headers(res_5, true).await?;
    Ok(())
}
#[tokio::test]
async fn check_connection_6() -> Result<(), Box<dyn std::error::Error>> {
    let res_6 = check_client_connection(GIBBERISH.into()).await?;
    status_and_headers(res_6, true).await?;
    Ok(())
}

//THIS TEST COULD BE BETTER, FAILS BEFORE CONNECTION
#[tokio::test]
async fn check_connection_7() {
    let res_7 = check_client_connection(STARS.into()).await;
    assert!(res_7.is_err());
}
#[tokio::test]
async fn check_connection_8() -> Result<(), Box<dyn std::error::Error>> {
    let res_8 = check_client_connection(ROMEO.into()).await?;
    status_and_headers(res_8, true).await?;
    Ok(())
}
#[tokio::test]
async fn check_connection_9() -> Result<(), Box<dyn std::error::Error>> {
    let res_9 = check_client_connection(DUNE.into()).await?;
    status_and_headers(res_9, true).await?;
    Ok(())
}
#[tokio::test]
async fn check_bad_query() -> Result<(), Box<dyn std::error::Error>> {
    let res = autocomplete_func(TERRIBLE_QUERY.into()).await;
    assert!(res.is_err());
    Ok(())
}

#[tokio::test]
async fn check_no_results() -> Result<(), Box<dyn std::error::Error>> {
    let res = autocomplete_func(GIBBERISH.into()).await;
    log::info!("{:?}", res);
    assert!(res.is_ok());
    let movies: Vec<MovieDisplay> = res.unwrap();
    assert!(movies.is_empty());
    Ok(())
}

#[tokio::test]
async fn check_weird_chars() -> Result<(), Box<dyn std::error::Error>> {
    let res = autocomplete_func(ROMEO.into()).await;
    log::info!("{:?}", &res);
    assert!(res.is_ok());
    let movies = res.unwrap();
    log::info!("{:?}", &movies);
    assert!(!movies.is_empty());

    let res_2 = autocomplete_func(NUMBERS.into()).await;
    assert!(res_2.is_ok());
    let movies: Vec<MovieDisplay> = res_2.unwrap();
    assert!(!movies.is_empty());

    Ok(())
}

#[tokio::test]
async fn check_fail_request() {
    let res = autocomplete_func(TERRIBLE_QUERY.into()).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn check_good_request_1() {}

#[tokio::test]
async fn check_good_request_2() {}

#[tokio::test]
async fn check_good_request_3() {}

#[tokio::test]
async fn verify_image_1() {}

#[tokio::test]
async fn verify_image_2() {}

#[tokio::test]
async fn verify_image_3() {}
