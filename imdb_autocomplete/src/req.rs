use reqwest::Url;
use shared_stuff::ImdbQuery;

use log::{debug, error, info, log, trace, warn};

const IMDB_URL: &str = "https://sg.media-imdb.com/suggests";

// WHICH CHARACTERS ARE ACTUALLY VALID? AND WHICH ARE NOT?
//
// CHANGED STRING TO &STR, CURRENTLY RETURNS AN EMPTY STRING IF THE FULL QUERY DOES
// NOT CONTAIN ANY LETTERS OR NUMBERS, BUT MAYBE AN ERROR IS BETTER.
fn filter_search_term(search_term: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut indices_iter = search_term
        .char_indices()
        .skip_while(|(_x, y)| !y.is_alphanumeric());
    if let Some((index, _c)) = indices_iter.next() {
        Ok(search_term[index..].to_string())
    } else {
        Err("empty search term".into())
    }
}

// I DONT KNOW WHY THERE IS AN UNDERSCORE, BUT I'M SURE WE WILL FIND OUT LATER
//
// https://sg.media-imdb.com/suggests/r/Romio%20+%20Juliet.json -> imdb$Romio___Juliet
fn build_search_url(filtered_search_term: String) -> Result<Url, Box<dyn std::error::Error>> {
    let mut temp = [0; 2];
    let first_letter = match filtered_search_term.chars().next() {
        Some(x) => x.to_ascii_lowercase().encode_utf8(&mut temp),
        None => "_",
    };

    let url_str = format!(
        "{}/{}/{}.json",
        IMDB_URL, first_letter, filtered_search_term
    );
    let final_url = Url::parse(&url_str)?;

    Ok(final_url)
}

pub fn build_url(search_term: ImdbQuery) -> Result<(String, Url), Box<dyn std::error::Error>> {
    let search_term = filter_search_term(search_term.query)?;
    //info!("search term is: {}", &search_term);
    let url = build_search_url(search_term.clone())?;
    Ok((search_term, url))
}
