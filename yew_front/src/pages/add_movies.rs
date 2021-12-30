use crate::utils::auth_flow;
use crate::GET_GROUP_MOVIES_URL;
use crate::SEARCH_URL;
use anyhow::Result;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::add_movies_stuff::UserGroup;
use shared_stuff::groups_stuff::BasicUsername;
use shared_stuff::ImdbQuery;
use shared_stuff::MovieDisplay;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;

use crate::utils::get_search_results;
use crate::utils::image_processing;

#[derive(Debug)]
pub enum AddMoviesMsg {
    Noop,
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddedMovies(Vec<MovieDisplay>),
    Error(String),
}
pub struct AddMovies {
    pub autocomplete_movies: Vec<MovieDisplay>,
    pub added_movies: Vec<MovieDisplay>,
    pub group_name: String,
}

impl Component for AddMovies {
    type Message = AddMoviesMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        let group_name = String::from("");
        Self {
            autocomplete_movies: vec![],
            added_movies: vec![],
            group_name,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AddMoviesMsg::*;
        let link_clone = ctx.link().clone();
        match msg {
            Noop => {}
            AddedMovies(movies) => {}
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
                self.autocomplete_movies = movies;
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
