use crate::pages::add_movies_html::get_search_results;
use crate::{GET_GROUP_MOVIES_URL, SAVE_GROUP_MOVIES_URL, SEARCH_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode};
use shared_stuff::add_movies_stuff::UserGroup;
use shared_stuff::groups_stuff::{GroupForm, GroupMoviesForm, UserGroupsJson};
use shared_stuff::{ImdbQuery, MovieDisplay};
use std::collections::{HashMap, HashSet};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

pub async fn get_all_added_movies(username: String, group_name: String) -> Result<Vec<String>> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = Request::post(&GET_GROUP_MOVIES_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    let groups: UserGroupsJson = resp.json().await?;
    log::info!("{:?}", &groups.groups);
    Ok(groups.groups)
}

pub async fn save_movies_request(
    username: String,
    group_name: String,
    current_movies: HashSet<MovieDisplay>,
) -> Result<()> {
    let json_body = serde_json::to_string(&GroupMoviesForm {
        username,
        group_name,
        current_movies,
    })?;
    let resp = Request::post(&SAVE_GROUP_MOVIES_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    Ok(())
}

#[derive(Debug)]
pub enum AddMoviesMsg {
    SaveMovies,
    Noop,
    GetMovies,
    DeleteEntry(MouseEvent),
    UpdateCurrentMovies(Vec<MovieDisplay>),
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
    fn create(ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        ctx.link().send_message(AddMoviesMsg::GetMovies);
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
        let storage = LocalStorage::raw();
        let username = storage
            .get("username")
            .expect("storage issue")
            .expect("option problem");
        let group_name = storage
            .get("current_group")
            .expect("storage issue")
            .expect("option problem");
        let movies = storage.get("added_movies").expect("storage issue");
        let mut current_movies: HashSet<MovieDisplay> = HashSet::new();
        if let Some(movies) = movies {
            current_movies = serde_json::from_str(&movies).expect("serialization error");
        }
        match msg {
            UpdateCurrentMovies(movies) => {
                let movie_string = serde_json::to_string(&movies).expect("serialization error");
                storage
                    .set("added_movies", &movie_string)
                    .expect("storage problem");
                for i in movies {
                    self.added_movies.insert(i.movie_id.clone(), i.clone());
                }
                log::info!("added_movies: {:?}", &self.added_movies);
            }
            SaveMovies => {
                let movies = self.added_movies.clone();
                link_clone.send_future(async move {
                    let current: HashSet<MovieDisplay> =
                        HashSet::from_iter(movies.values().cloned());
                    save_movies_request(username, group_name, current).await;
                    AddMoviesMsg::Noop
                })
            }
            GetMovies => link_clone.send_future(async move {
                let current = get_group_movies(username, group_name).await.expect("umm");
                AddMoviesMsg::UpdateCurrentMovies(current)
            }),
            Noop => {}
            DeleteEntry(movie) => {
                if let Some(elem) = movie.target_dyn_into::<HtmlElement>() {
                    log::trace!("inside AddMovie");
                    log::trace!("{:?}", &elem.title());
                    log::trace!("checking added movies: {:?}", &self.added_movies);
                    self.added_movies.remove(&elem.title());
                }
            }
            AddMovie(movie) => {
                let storage = LocalStorage::raw();
                if let Some(elem) = movie.target_dyn_into::<HtmlElement>() {
                    log::trace!("inside AddMovie");
                    log::trace!("checking current movies: {:?}", &self.autocomplete_movies);
                    log::trace!("current element: {:?}", &elem.title());
                    let lookup_title = &elem.title();
                    let movie = self
                        .autocomplete_movies
                        .get(lookup_title)
                        .expect("not here");
                    self.added_movies
                        .insert(lookup_title.clone(), movie.clone());
                    let movie_vec = self.added_movies.values().collect::<Vec<_>>();
                    let json_movies = serde_json::to_string(&movie_vec).expect("json issue");
                    storage
                        .set("added_movies", &json_movies)
                        .expect("storage issue");
                }
            }
            QueryAutocomplete(text) => {
                // Shouldn't do it if the text is empty, but handle this better probably...
                if text.current_target().is_some() {
                    link_clone.clone().send_future(async move {
                        if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                            let query = ImdbQuery {
                                query: elem.value(),
                            };

                            match get_search_results(&SEARCH_URL, query).await {
                                Ok(resp) => AddMoviesMsg::UpdateAutocomplete(resp),
                                Err(err_msg) => {
                                    link_clone.clone().send_message(UpdateAutocomplete(vec![]));
                                    AddMoviesMsg::Error(err_msg.to_string())
                                }
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
                    self.autocomplete_movies.insert(i.movie_id.clone(), i);
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
            {self.save_movies(ctx)}
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

//pub async fn get_all_groups(username: String) -> Result<Vec<String>> {
//let json_body = serde_json::to_string(&BasicUsername { username })?;
//let resp: Vec<String> = Request::post(&GET_GROUP_MOVIES_URL)
//.header("content-type", "application/json; charset=UTF-8")
//.mode(RequestMode::Cors)
//.body(json_body)
//.send()
//.await?
//.json()
//.await?;
//log::info!("{:?}", &resp);
//Ok(resp)
//}
