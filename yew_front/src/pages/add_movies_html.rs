use crate::pages::add_movies::AddMovies;
use crate::pages::add_movies::AddMoviesMsg;
use crate::utils::image_processing;
use yew::prelude::*;

impl AddMovies {
    pub fn search_results(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(|e| AddMoviesMsg::AddMovie(e));
        {
            self.autocomplete_movies
                .values()
                // Still not handling the no images nicely?
                .map(|movie| {
                    if movie.movie_images.is_some() {
                        let formatted =
                            format!("{} {}", &movie.movie_title, &movie.movie_year.unwrap());
                        html! {
                        <li class="search_results_li" title = {movie.movie_title.clone()}
                        onclick={callback.clone()}>
                            {&formatted}
                        <img src={image_processing(movie.movie_images.as_ref())}
                        title = {movie.movie_title.clone()}/>
                        </li>}
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
