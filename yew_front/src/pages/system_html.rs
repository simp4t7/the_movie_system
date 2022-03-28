use crate::pages::system::{System, SystemMsg};
use shared_stuff::db_structs::GroupData;
use shared_stuff::shared_structs::{SystemState, YewMovieDisplay};
use std::collections::VecDeque;

use shared_stuff::imdb_structs::ImageData;
use yew::prelude::*;

impl System {
    pub fn view_group_id(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }

    pub fn user_customized_view(&self, ctx: &Context<Self>) -> Html {
        match &self.loaded {
            true => {
                html! {
                    <div>
                    { self.view_group_data(ctx, &self.group_data) }
                    </div>
                }
            }
            false => {
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
                <li>{format!("system status: {:?}", group_data.system_state)}</li>
                <li>{format!("current turn: {:?}", group_data.turn)}</li>
            </div>
        }
    }

    pub fn search_results(&self, ctx: &Context<Self>) -> Html {
        //let callback = ctx.link().callback(SystemMsg::AddMovie);
        {
            log::info!("autocomplete_movies: {:?}", &self.autocomplete_movies);
            self.autocomplete_movies
                .iter()
                // Still not handling the no images nicely?
                .map(|movie| {
                    let movie_clone = movie.clone();
                    let imdb_link = format!("https://imdb.com/title/{}", &movie.movie_id);
                    
                        html! {
                        <a class="panel-block px-0">
                        <div class="column p-0 is-narrow">
                        <img style="width: 80px;" src={image_processing(&movie.movie_images)}/>
                        </div>
                        <div class="column is-flex-direction-column p-0" id = {movie.movie_id.clone()}>
                        <li class="content mb-1 ml-3 is-size-5 is-size-6-mobile ellipsis is-ellipsis-1">
                        {&movie.movie_title}
                        </li>
                        <li class="content mb-1 ml-3 is-size-6 is-size-7-mobile">
                        {&movie.movie_year}
                        </li>
                        <li class="content mb-1 ml-3 is-size-6 is-size-7-mobile">
                        {&movie.movie_stars}
                        </li>
                        <div class="columns is-mobile">
                        <div class="column ml-3">
                        <button
                            class="button is-primary is-small is-fullwidth"
                        onclick={&ctx.link().callback(move |_| SystemMsg::AddMovie(movie_clone.clone()))}>
                            { "Add to System" }
                        </button>
                        </div>
                        <div class="column mr-3">
                        <a
                            class="button is-primary is-small is-fullwidth"
                            target="_blank"
                            rel="noopener noreferrer"
                            href={imdb_link}>
                            { "Visit IMDB" }
                        </a>
                        </div>
                        </div>
                        </div>
                        </a>
                    } 
                })
                .collect::<Html>()
        }
    }
    pub fn display_current_members(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="columns">
            {self.current_members(ctx)}
            </div>
        }
    }
    pub fn current_members(&self, ctx: &Context<Self>) -> Html {
        self.group_data
            .members
            .keys()
            .map(|member| {
                let color = &self.group_data.members.get(member).expect("no member?").color;
                html! {
                    <div class="column is-2 is-flex is-flex-direction-rows">
                    <div class="column p-0">
                    <button class={format!("button is-fullwidth p-0 {}", color)}>
                    {member}
                    </button>
                    </div>
                    {

                        match self.group_data.members.get(&member.clone()).expect("umm").ready_status {
                        true => html!{
                            <div class="button" style="border-style: none;">
                            <span class="icon">
                            <i class="material-icons md-48">{"check_circle_outline"}</i>
                            </span>
                            </div>
                        },
                        false => html! {
                            <div class="button" style="border-style: none;">
                            <span class="icon">
                            <i class="material-icons md-48">{"block"}</i>
                            </span>
                            </div>
                        },
                    }     }
                    </div>
                }
            })
            .collect::<Html>()
    }
    pub fn full_search_html(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <div class="container">
            <div class="column is-12 is-flex-direction-rows">
            {self.display_current_members(ctx)}
            </div>
            </div>
            <div class="container">
            <div class="column is-8 mx-auto">
            {self.search_bar(ctx)}
            </div>
            {self.add_stuff(ctx)}
            </div>
            </div>
        }
    }
    pub fn search_bar(&self, ctx: &Context<Self>) -> Html {
        //if let Some(data) = self.group_data.clone() {
        match self.group_data.system_state {
            SystemState::AddingMovies => html! {
                        <div>
                        <div>
                            <p class="control has-icons-left">
                            <input class="input" type="text" placeholder="Movie Search"
                            maxlength=50
                            oninput={ctx.link().callback(SystemMsg::QueryAutocomplete)}/>
                            <span class="icon is-small is-left">
                            <i class="material-icons">{"search"}</i>
                            </span>
                            </p>
                        </div>
                            <ul style="position:absolute; background-color:blue;">
                            <div class="container" style="position:relative; width:100%;">
                            {self.search_results(ctx)}
                            </div>
                            </ul>
                        </div>
            },

            SystemState::SystemStarted => html! {},
            SystemState::Finished => html! {},
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

    pub fn system_status_bar(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div class="container column is-9 is-flex is-flex-direction-horizontal">
            <span class="icon my-auto has-text-success step-progress-icon" style="left: 1em;">
            <i class="material-icons md-48">{"check_circle"}</i>
            </span>
            <progress class="progress my-auto" value="100" max="100"></progress>
            <span class="icon my-auto has-text-success step-progress-icon" style="right: 1em;">
            <i class="material-icons md-48">{"check_circle"}</i>
            </span>
            <progress class="progress my-auto" style="position:relative; right: 2em;" value="100" max="100"></progress>
            <span class="icon my-auto has-text-success step-progress-icon" style="right: 3em;">
            <i class="material-icons md-48">{"check_circle"}</i>
            </span>
            <progress class="progress my-auto" style="position:relative; right: 4em;" value="100" max="100"></progress>
            <span class="icon my-auto has-text-success step-progress-icon" style="right: 5em;">
            <i class="material-icons md-48">{"check_circle"}</i>
            </span>
        </div>
        }
    }

    pub fn ready_status_buttons(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <button
                class="button is-primary"
                onclick={&ctx.link().callback(|_|SystemMsg::SetReady)}>
                { "I'm ready!" }
            </button>
            <button
                class="button is-primary"
                onclick={&ctx.link().callback(|_|SystemMsg::UnsetReady)}>
                { "Nah, not ready" }
            </button>
            </div>

        }
    }

    pub fn delete_movie_button(&self, ctx: &Context<Self>, movie: YewMovieDisplay) -> Html {
        if self.group_data.system_state == SystemState::AddingMovies
            && self.username == movie.added_by
            && self
                .group_data
                .members
                .get(&self.username)
                .expect("hashmap problem")
                .ready_status
                != true
        {
            html! {
            <button
                class="delete entry" title = {movie.movie_title.clone()}
                onclick={&ctx.link().callback(move|_| SystemMsg::DeleteEntry(movie.clone()))}>
                { "delete entry" }
            </button>  }
        } else if self.group_data.system_state == SystemState::SystemStarted
            && self.group_data.turn == self.username
        {
            html! {
            <button
                class="delete entry" title = {movie.movie_title.clone()}
                onclick={&ctx.link().callback(move|_| SystemMsg::DeleteEntryChangeTurn(movie.clone()))}>
                { "delete entry" }
            </button>  }
        } else if self.group_data.system_state == SystemState::Finished {
            html! {}
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
                    let _formatted = format!("{} {}", &movie.movie_title, &movie.movie_year);
                    html! {
                        
                        <a class="panel-block px-0">
                        <div class="container">
                        <div class="column p-0 is-narrow">
                        <img style="width: 80px;" src={image_processing(&movie.movie_images)}/>
                        </div>
                        <div class="column is-flex-direction-column p-0" id = {movie.movie_id.clone()}>
                        <li class="content mb-1 ml-3 is-size-5 is-size-6-mobile ellipsis is-ellipsis-1">
                        {&movie.movie_title}
                        </li>
                        <li class="content mb-1 ml-3 is-size-6 is-size-7-mobile">
                        {&movie.movie_year}
                        </li>
                        <li class="content mb-1 ml-3 is-size-6 is-size-7-mobile">
                        {&movie.movie_stars}
                        </li>

                        {   self.delete_movie_button(ctx, movie) }
                        </div>
                        <div class="columns is-mobile">
                        <div class="column ml-3">
                        </div>
                        </div>
                        </div>
                        </a>



                            //<div class="temp_movies">
                            //<img class="search_results_image"
                                //src={image_processing(&movie.movie_images)}/>
                            //<ul>
                            //<li> {&movie.movie_title} </li>
                            //<li> {&movie.movie_year} </li>
                            //<li> {format!("added by: {}", &movie.added_by)} </li>
                            //</ul>
                            //{   self.delete_movie_button(ctx, movie) }
                            //</div>

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

pub fn image_processing(image: &ImageData) -> String {
    let mut image_url = image.url.to_owned();
    assert!(&image_url[image_url.len() - 4..] == ".jpg");
    image_url.truncate(image_url.len() - 4);
    image_url.push_str("QL75_UX80_CR0,5,80,120_.jpg");
    image_url
}
