use crate::pages::add_movies::{AddMovies, AddMoviesMsg};
use anyhow::Result;
use reqwasm::http::{Request, RequestMode};
use shared_stuff::{ImageData, ImdbQuery, MovieDisplay};
use yew::prelude::*;

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

pub async fn get_search_results(url: &str, body: ImdbQuery) -> Result<Vec<MovieDisplay>> {
    let imdbquery = serde_json::to_string(&body)?;
    let resp = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(imdbquery)
        .send()
        .await?
        .json()
        .await?;
    log::info!("movie resp: {:?}", &resp);
    Ok(resp)
}

impl AddMovies {
    pub fn search_results(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|e| AddMoviesMsg::AddMovie(e));
        {
            log::info!("autocomplete_movies: {:?}", &self.autocomplete_movies);
            self.autocomplete_movies
                .values()
                // Still not handling the no images nicely?
                .map(|movie| {
                    if movie.movie_images.is_some() {
                        html! {
                        <div class="search_results_div"
                            title = {movie.movie_id.clone()}
                            onclick={callback.clone()}>
                        <img class="search_results_image"
                            src={image_processing(movie.movie_images.as_ref())}
                            title = {movie.movie_id.clone()}/>
                        <ul>
                        <li class="search_results_info"
                            title = {movie.movie_id.clone()}>
                        {&movie.movie_title}
                        </li>
                        <li class="search_results_info"
                            title = {movie.movie_id.clone()}>
                        {&movie.movie_year.unwrap()}
                        </li>
                        <li class="search_results_info"
                            title = {movie.movie_id.clone()}>
                        {&movie.movie_stars}
                        </li>
                        </ul>
                        </div>
                        }
                    } else {
                        html! {
                        <li>
                            {&movie.movie_title}
                            {&movie.movie_year.unwrap()}
                        </li>}
                    }
                })
                .collect::<Html>()
        }
    }
    pub fn search_bar(&self, ctx: &Context<Self>) -> Html {
        html! {
                    <div class="movie_search_div">
                        <input
                        class="movie_search"
                        placeholder="movie search"
                        maxlength=50
                        oninput={ctx.link().callback(AddMoviesMsg::QueryAutocomplete)}
                        />
                    <div class="search_results">
                    <ul>
                    {self.search_results(ctx)}
                    </ul>
                    </div>
                    </div>
        }
    }

    pub fn add_stuff(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <ul>
                {self.added_movies(ctx)}
                </ul>
            </div>

        }
    }

    pub fn added_movies(&self, ctx: &Context<Self>) -> Html {
        {
            self.added_movies
                .values()
                // Still not handling the no images nicely?
                .map(|movie| {
                    let formatted =
                        format!("{} {}", &movie.movie_title, &movie.movie_year.unwrap());
                    html! {
                        <div>
                            <li class="search_results_li" >
                                {&formatted}
                                <img src={image_processing(movie.movie_images.as_ref())}/>
                            </li>
                    <button
                        class="delete entry" title = {movie.movie_title.clone()}
                        onclick={&ctx.link().callback(|e| AddMoviesMsg::DeleteEntry(e))}>
                        { "delete entry" }
                    </button>
                        </div>

                            }
                })
                .collect::<Html>()
        }
    }

    pub fn save_movies(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button
                    class="save_movies"
                    onclick={&ctx.link().callback(|_| AddMoviesMsg::SaveMovies)}>
                    { "Save Movies" }
                </button>
            </div>

        }
    }
}
