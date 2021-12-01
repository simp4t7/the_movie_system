use reqwest::Url;

//#[cfg(test)]
//mod tests {
//use super::*;

//#[test]
//fn test_2_word() {
//let input_str = "Star wars";
//let (_, url) = build_url(input_str).unwrap();
//let expected = Url::parse("https://sg.media-imdb.com/suggests/s/Star wars.json").unwrap();
//assert_eq!(url, expected);
//}
//#[test]
//fn test_accent_marks() {
//let input_str = "Vietnam aux lèvres";
//let (_, url) = build_url(input_str).unwrap();
//let expected =
//Url::parse("https://sg.media-imdb.com/suggests/v/Vietnam aux lèvres.json").unwrap();
//assert_eq!(url, expected);
//}
//#[test]
//fn test_symbol() {
//let input_str = "Romeo + Juliet";
//let (_, url) = build_url(input_str).unwrap();
//let expected =
//Url::parse("https://sg.media-imdb.com/suggests/r/Romeo + Juliet.json").unwrap();
//assert_eq!(url, expected);
//}
//#[test]
//fn test_weird_symbols() {
//let input_str = "!23";
//let (_, url) = build_url(input_str).unwrap();
//let expected = Url::parse("https://sg.media-imdb.com/suggests/2/23.json").unwrap();
//assert_eq!(url, expected);
//}
//#[test]
//fn test_no_letters_or_numbers() {
//let input_str = "@@#*)(*!)*";
//let (_, url) = build_url(input_str).unwrap();
//let expected = Url::parse("https://sg.media-imdb.com/suggests/_/.json").unwrap();
//assert_eq!(url, expected);
//}
//#[test]
//fn test_all_caps() {
//let input_str = "STAR WARS";
//let (_, url) = build_url(input_str).unwrap();
//let expected = Url::parse("https://sg.media-imdb.com/suggests/s/STAR WARS.json").unwrap();
//assert_eq!(url, expected);
//}
//}
const IMDB_URL: &str = "https://sg.media-imdb.com/suggests";

// WHICH CHARACTERS ARE ACTUALLY VALID? AND WHICH ARE NOT?
//
// CHANGED STRING TO &STR, CURRENTLY RETURNS AN EMPTY STRING IF THE FULL QUERY DOES
// NOT CONTAIN ANY LETTERS OR NUMBERS, BUT MAYBE AN ERROR IS BETTER.
fn filter_search_term(search_term: &str) -> Result<&str, Box<dyn std::error::Error>> {
    let mut indices_iter = search_term
        .char_indices()
        .skip_while(|(_x, y)| !y.is_alphanumeric());
    if let Some((index, _c)) = indices_iter.next() {
        Ok(&search_term[index..])
    } else {
        Err("empty search term".into())
    }
}

// I DONT KNOW WHY THERE IS AN UNDERSCORE, BUT I'M SURE WE WILL FIND OUT LATER
//
// https://sg.media-imdb.com/suggests/r/Romio%20+%20Juliet.json -> imdb$Romio___Juliet
fn build_search_url(filtered_search_term: &str) -> Result<Url, Box<dyn std::error::Error>> {
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

pub fn build_url(search_term: &str) -> Result<(&str, Url), Box<dyn std::error::Error>> {
    let search_term = filter_search_term(search_term)?;
    info!("search term is: {}", &search_term);
    let url = build_search_url(search_term)?;
    Ok((search_term, url))
}
