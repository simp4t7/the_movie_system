use crate::pages::system::{System, SystemMsg};
use shared_stuff::db_structs::{GroupData, SystemState};
use shared_stuff::YewMovieDisplay;

use shared_stuff::ImageData;
use yew::prelude::*;

impl System {
    pub fn view_group_id(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }

    pub fn user_customized_view(&self, ctx: &Context<Self>) -> Html {
        match &self.group_data {
            Some(group_data) => {
                html! {
                    <div>
                    { self.view_group_data(ctx, &group_data) }
                    </div>
                }
            }
            None => {
                html! {
                    <p>{format!("This group doesn't exist or you don't have the access to it.")}</p>
                }
            }
        }
    }

    fn view_group_data(&self, _ctx: &Context<Self>, group_data: &GroupData) -> Html {
        html! {
            <div>
                <p>{format!("group data is:")}</p>
                <li>{format!("Name: {}", group_data.group_name)}</li>
                <li>{format!("Members: {:?}", group_data.members)}</li>
                <li>{format!("Date created: {:?}", group_data.date_created)}</li>
                <li>{format!("Movies watched: {:?}", group_data.movies_watched)}</li>
            </div>
        }
    }

    pub fn search_results(&self, ctx: &Context<Self>) -> Html {
        let callback = ctx.link().callback(SystemMsg::AddMovie);
        {
            log::info!("autocomplete_movies: {:?}", &self.autocomplete_movies);
            self.autocomplete_movies
                .values()
                // Still not handling the no images nicely?
                .map(|movie| {
                    if movie.movie_images.is_some() {
                        html! {
                        <div class="search_results_div"
                            id = {movie.movie_id.clone()}
                            onclick={callback.clone()}>
                        <img class="search_results_image"
                            src={image_processing(movie.movie_images.as_ref())}
                            id = {movie.movie_id.clone()}/>
                        <ul>
                        <li class="search_results_info"
                            id = {movie.movie_id.clone()}>
                        {&movie.movie_title}
                        </li>
                        <li class="search_results_info"
                            id = {movie.movie_id.clone()}>
                        {&movie.movie_year.unwrap()}
                        </li>
                        <li class="search_results_info"
                            id = {movie.movie_id.clone()}>
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
                        oninput={

                            ctx.link().callback(SystemMsg::QueryAutocomplete)
                        }
                        />
                    <div class="search_results">
                    <ul class = "ul_search">
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
                {self.save_movies(ctx)}
            </div>

        }
    }

    pub fn ready_status_buttons(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button
                class="ready_status"
                onclick={&ctx.link().callback(|_|SystemMsg::SetReady)}>
                { "I'm ready!" }
            </button>
            <button
                class="ready_status"
                onclick={&ctx.link().callback(|_|SystemMsg::UnsetReady)}>
                { "Nah, not ready" }
            </button>
            </div>

        }
    }

    pub fn delete_movie_button(&self, ctx: &Context<Self>, movie: YewMovieDisplay) -> Html {
        if let Some(data) = self.group_data.clone() {
            if data.system_state == SystemState::AddingMovies && self.username == movie.added_by {
                html! {
                <button
                    class="delete entry" title = {movie.movie_title.clone()}
                    onclick={&ctx.link().callback(move|_| SystemMsg::DeleteEntry(movie.clone()))}>
                    { "delete entry" }
                </button>  }
            } else {
                html! {}
            }
        } else {
            html! {}
        }
    }

    pub fn added_movies(&self, ctx: &Context<Self>) -> Html {
        {
            log::info!("self.current_movies: {:?}", &self.current_movies);
            let current = self.current_movies.clone();
            current
                .iter()
                .cloned()
                .map(|movie| {
                    let _formatted =
                        format!("{} {}", &movie.movie_title, &movie.movie_year.unwrap());
                    html! {
                            <div class="temp_movies">
                            <img class="search_results_image"
                                src={image_processing(movie.movie_images.as_ref())}/>
                            <ul>
                            <li> {&movie.movie_title} </li>
                            <li> {&movie.movie_year.unwrap()} </li>
                            <li> {format!("added by: {}", &movie.added_by)} </li>
                            </ul>
                            {   self.delete_movie_button(ctx, movie) }
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
                    onclick={&ctx.link().callback(|_| SystemMsg::SaveMovies)}>
                    { "Save Movies" }
                </button>
            </div>

        }
    }
}

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
