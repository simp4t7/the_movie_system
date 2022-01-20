use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OmdbRatings {
    Source: String,
    Value: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OmdbStruct {
    Title: String,
    Year: String,
    Rated: String,
    Released: String,
    Runtime: String,
    Genre: String,
    Director: String,
    Writer: String,
    Actors: String,
    Plot: String,
    Language: String,
    Country: String,
    Awards: String,
    Poster: String,
    Ratings: Vec<OmdbRatings>,
    Metascore: String,
    imdbRating: String,
    imdbVotes: String,
    imdbID: String,
    Type: String,
    DVD: String,
    BoxOffice: String,
    Production: String,
    Website: String,
    Response: String,
}
