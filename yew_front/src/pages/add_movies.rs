use crate::GET_GROUP_MOVIES_URL;
use crate::SEARCH_URL;
use anyhow::Result;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::add_movies_stuff::UserGroup;
use shared_stuff::groups_stuff::BasicUsername;
use shared_stuff::ImdbQuery;
use shared_stuff::MovieDisplay;
use web_sys::Element;
use web_sys::HtmlElement;
//use web_sys::HtmlImageElement;
use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;

use crate::utils::get_search_results;

#[derive(Debug)]
pub enum AddMoviesMsg {
    Noop,
    DeleteEntry(MouseEvent),
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddMovie(MouseEvent),
    Error(String),
}
pub struct AddMovies {
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub added_movies: HashMap<String, MovieDisplay>,
    pub group_name: String,
}

impl Component for AddMovies {
    type Message = AddMoviesMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        let group_name = String::from("");
        Self {
            autocomplete_movies: HashMap::new(),
            added_movies: HashMap::new(),
            group_name,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AddMoviesMsg::*;
        let link_clone = ctx.link().clone();
        match msg {
            Noop => {}
            DeleteEntry(movie) => {
                if let Some(elem) = movie.target_dyn_into::<HtmlElement>() {
                    log::info!("inside AddMovie");
                    log::info!("{:?}", &elem.title());
                    log::info!("checking added movies: {:?}", &self.added_movies);
                    self.added_movies.remove(&elem.title());
                }
            }
            AddMovie(movie) => {
                if let Some(elem) = movie.target_dyn_into::<HtmlElement>() {
                    log::info!("inside AddMovie");
                    log::info!("checking current movies: {:?}", &self.autocomplete_movies);
                    log::info!("current element: {:?}", &elem.title());
                    let lookup_title = &elem.title();
                    let movie = self
                        .autocomplete_movies
                        .get(lookup_title)
                        .expect("not here");
                    self.added_movies
                        .insert(lookup_title.clone(), movie.clone());
                }
            }
            QueryAutocomplete(text) => {
                // Shouldn't do it if the text is empty, but handle this better probably...
                if text.current_target().is_some() {
                    link_clone.send_future(async move {
                        if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                            let query = ImdbQuery {
                                query: elem.value(),
                            };

                            match get_search_results(&SEARCH_URL, query).await {
                                Ok(resp) => AddMoviesMsg::UpdateAutocomplete(resp),
                                Err(err_msg) => AddMoviesMsg::Error(err_msg.to_string()),
                            }
                        } else {
                            AddMoviesMsg::Noop
                        }
                    });
                }
            }

            AddMoviesMsg::UpdateAutocomplete(movies) => {
                log::info!("{:?}", &movies);
                self.autocomplete_movies.clear();
                for i in movies {
                    self.autocomplete_movies.insert(i.movie_title.clone(), i);
                }

                //self.autocomplete_movies = movies;
            }
            AddMoviesMsg::Error(err_msg) => {
                log::info!("{:?}", &err_msg);
            }
        };
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            {self.search_bar(ctx)}
            {self.add_stuff(ctx)}
        </div>}
    }
}

pub async fn get_group_movies(username: String, group_name: String) -> Result<Vec<MovieDisplay>> {
    let json_body = serde_json::to_string(&UserGroup {
        username,
        group_name,
    })?;
    let resp: Vec<MovieDisplay> = Request::post(&GET_GROUP_MOVIES_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?
        .json()
        .await?;
    log::info!("{:?}", &resp);
    Ok(resp)
}

pub async fn get_all_groups(username: String) -> Result<Vec<String>> {
    let json_body = serde_json::to_string(&BasicUsername { username })?;
    let resp: Vec<String> = Request::post(&GET_GROUP_MOVIES_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?
        .json()
        .await?;
    log::info!("{:?}", &resp);
    Ok(resp)
}
