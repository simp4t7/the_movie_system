use shared_stuff::imdb_structs::{JsonQuery, MediaType, MovieInfo};
use shared_stuff::shared_structs::MovieDisplay;
use uuid::Uuid;

use log::trace;

// MAYBE MAKE THE FALLIBLE WITH RESULT TYPE, MIGHT BE NICE.
fn trim_body_string(body_string: String, query: String) -> String {
    let start = format!("imdb${}", &query);
    // check if body_string starts with start = {"imdb+query"}
    let start_len = start.len() + 1;
    let end_len = body_string.len() - 1;
    body_string[start_len..end_len].to_string()
}

fn serialize_raw_json(input: String) -> Result<Vec<MovieDisplay>, Box<dyn std::error::Error>> {
    let mut result_vec: Vec<MovieDisplay> = vec![];
    let json_query: JsonQuery = serde_json::from_str(&input)?;
    let temp_movie_vec: Option<Vec<MovieInfo>> = json_query.d;

    if let Some(movie_vec) = temp_movie_vec {
        for movie in movie_vec {
            trace!("{:?}", &movie.q);
            let media_type = MediaType::new(movie.q);
            let mut movie_stars = String::from("");
            if let Some(stars) = movie.s {
                movie_stars = stars;
            }
            trace!("{:?}", &media_type);
            match media_type {
                MediaType::Movie | MediaType::MiniSeries | MediaType::TV => {
                    let movie_images = serde_json::from_value(movie.i.expect("no image"))?;
                    let movie_year = movie.y.expect("no year");

                    //let movie_images: ImageData = match movie.i {
                    //Some(image_data) => serde_json::from_value(image_data).unwrap(),
                    //None => None,
                    //};
                    let movie_id = movie.id.expect("no id");

                    //if movie_images.is_some() && movie.y.is_some() {
                    result_vec.push(MovieDisplay {
                        movie_id,
                        movie_title: movie.l.unwrap(),
                        movie_year,
                        movie_stars,
                        movie_images,
                    });
                    //}
                }
                _ => {}
            }
        }
    }

    Ok(result_vec)
}

pub fn make_movie_display(
    body_string: String,
    search_term: String,
) -> Result<Vec<MovieDisplay>, Box<dyn std::error::Error>> {
    let trimmed_json = trim_body_string(body_string, search_term);
    trace!("trimmed json string is: {}", &trimmed_json);
    let movie_display_vec = serialize_raw_json(trimmed_json)?;
    Ok(movie_display_vec)
}
