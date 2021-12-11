use ctor::ctor;
use imdb_autocomplete::req::build_url;
use reqwest::Url;

use log::{debug, error, info, log, trace, warn};

#[ctor]
fn load_logger() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
}

#[test]
fn test_2_word() {
    let input_str = "Star wars";
    info!("input string is {:?}", &input_str);
    let (_, url) = build_url(input_str.into()).unwrap();
    let expected = Url::parse("https://sg.media-imdb.com/suggests/s/Star wars.json").unwrap();
    assert_eq!(url, expected);
}
#[test]
fn test_accent_marks() {
    let input_str = "Vietnam aux lèvres";
    info!("input string is {:?}", &input_str);
    let (_, url) = build_url(input_str.into()).unwrap();
    let expected =
        Url::parse("https://sg.media-imdb.com/suggests/v/Vietnam aux lèvres.json").unwrap();
    assert_eq!(url, expected);
}
#[test]
fn test_symbol() {
    let input_str = "Romeo + Juliet";
    info!("input string is {:?}", &input_str);
    let (_, url) = build_url(input_str.into()).unwrap();
    let expected = Url::parse("https://sg.media-imdb.com/suggests/r/Romeo + Juliet.json").unwrap();
    assert_eq!(url, expected);
}
#[test]
fn test_weird_symbols() {
    let input_str = "!23";
    info!("input string is {:?}", &input_str);
    let (_, url) = build_url(input_str.into()).unwrap();
    let expected = Url::parse("https://sg.media-imdb.com/suggests/2/23.json").unwrap();
    assert_eq!(url, expected);
}
#[test]
fn test_no_letters_or_numbers() {
    let input_str = "@@#*)(*!)*";
    info!("input string is {:?}", &input_str);
    let url_result = build_url(input_str.into());
    assert!(url_result.is_err());
}
#[test]
fn test_all_caps() {
    let input_str = "STAR WARS";
    info!("input string is {:?}", &input_str);
    let (_, url) = build_url(input_str.into()).unwrap();
    let expected = Url::parse("https://sg.media-imdb.com/suggests/s/STAR WARS.json").unwrap();
    assert_eq!(url, expected);
}
