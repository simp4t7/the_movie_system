use crate::pages::add_movies::AddMovies;
use crate::pages::add_movies::AddMoviesMsg;
use crate::utils::image_processing;
use yew::prelude::*;

impl AddMovies {
    pub fn search_results(&self, _ctx: &Context<Self>) -> Html {
        {
            self.autocomplete_movies
                .iter()
                // Still not handling the no images nicely?
                .map(|movie| {
                    if movie.movie_images.is_some() {
                        html! {
                        <li class="search_results_li">
                            {&movie.movie_title}
                            {&movie.movie_year.unwrap()}
                            <img src={image_processing(movie.movie_images.as_ref())}/>
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

    pub fn added_movies(&self, _ctx: &Context<Self>) -> Html {
        {
            self.added_movies
                .iter()
                // Still not handling the no images nicely?
                .map(|movie| {
                    html! {
                    <li class="search_results_li">
                        {&movie.movie_title}
                        {&movie.movie_year.unwrap()}
                        <img src={image_processing(movie.movie_images.as_ref())}/>
                    </li>}
                })
                .collect::<Html>()
        }
    }
    pub fn display_chosen_movies(&self, _ctx: &Context<Self>) -> Html {
        {
            self.added_movies
                .iter()
                // Still not handling the no images nicely?
                .map(|movie| {
                    html! {
                    <li class="search_results_li">
                        {&movie.movie_title}
                        {&movie.movie_year.unwrap()}
                        <img src={image_processing(movie.movie_images.as_ref())}/>
                    </li>}
                })
                .collect::<Html>()
        }
    }
}
