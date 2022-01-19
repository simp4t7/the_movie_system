use crate::pages::add_movies_html::get_search_results;
use crate::{GET_GROUP_MOVIES_URL, SAVE_GROUP_MOVIES_URL, SEARCH_URL};
use anyhow::{anyhow, Result};
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode};
use shared_stuff::add_movies_stuff::UserGroup;
use shared_stuff::groups_stuff::{GroupForm, GroupMoviesForm, UserGroupsJson};
use shared_stuff::{ImdbQuery, MovieDisplay, YewMovieDisplay};
use std::collections::{HashMap, HashSet};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug)]
pub enum AddMoviesMsg {
    SaveMovies,
    Noop,
    GetMovies,
    DeleteEntry(MouseEvent),
    UpdateCurrentMovies(Vec<YewMovieDisplay>),
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddMovie(MouseEvent),
    Error(String),
}
pub struct AddMovies {
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub username: String,
    pub added_movies: HashMap<String, YewMovieDisplay>,
    pub group_name: String,
}

impl Component for AddMovies {
    type Message = AddMoviesMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        ctx.link().send_message(AddMoviesMsg::GetMovies);
        let storage = LocalStorage::raw();
        let mut username = String::from("");
        let mut group_name = String::from("");
        if let Some(user) = storage.get("username").expect("storage error") {
            username = user;
        }
        if let Some(group) = storage.get("current_group").expect("storage error") {
            group_name = group;
        }
        log::info!("username is: {:?}", &username);

        Self {
            autocomplete_movies: HashMap::new(),
            added_movies: HashMap::new(),
            group_name,
            username,
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AddMoviesMsg::*;
        let link_clone = ctx.link().clone();
        let storage = LocalStorage::raw();
        let username = storage.get("username").expect("storage issue");
        let group_name = storage.get("current_group").expect("storage issue");
        let movies = storage.get("added_movies").expect("storage issue");
        let mut current_movies: HashSet<MovieDisplay> = HashSet::new();
        if let Some(movies) = movies {
            current_movies = serde_json::from_str(&movies).expect("serialization error");
        }
        match msg {
            UpdateCurrentMovies(movies) => {
                log::info!("inside update current movies");
                let movie_string = serde_json::to_string(&movies).expect("serialization error");
                storage
                    .set("added_movies", &movie_string)
                    .expect("storage problem");
                log::info!("inside update: {:?}", &movies);
                for i in movies {
                    self.added_movies.insert(i.movie_id.clone(), i);
                }
                log::info!("added_movies: {:?}", &self.added_movies);
            }
            SaveMovies => {
                log::info!("inside save movies");
                let movies = self.added_movies.clone();
                link_clone.send_future(async move {
                    let current: HashSet<YewMovieDisplay> =
                        HashSet::from_iter(movies.values().cloned());
                    log::info!("current is: {:?}", &current);
                    let resp = save_movies_request(username, group_name, current).await;
                    log::info!("resp is: {:?}", &resp);
                    AddMoviesMsg::Noop
                })
            }
            GetMovies => link_clone.send_future(async move {
                let current = get_group_movies(username, group_name).await;
                match current {
                    Ok(current_movies) => AddMoviesMsg::UpdateCurrentMovies(current_movies),
                    Err(e) => AddMoviesMsg::Error(e.to_string()),
                }
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
                    log::info!("inside AddMovie");
                    log::info!("checking current movies: {:?}", &self.autocomplete_movies);
                    log::info!("current element: {:?}", &elem.title());
                    let lookup_title = &elem.id();
                    let movie = self
                        .autocomplete_movies
                        .get(lookup_title)
                        .expect("not here");
                    self.added_movies.insert(
                        lookup_title.clone(),
                        movie.clone().into_yew_display(self.username.clone()),
                    );
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
        <div class="add_movies_page">
            {self.search_bar(ctx)}
            {self.add_stuff(ctx)}
        </div>}
    }
}

pub async fn get_group_movies(
    username: Option<String>,
    group_name: Option<String>,
) -> Result<Vec<YewMovieDisplay>> {
    if let (Some(username), Some(group_name)) = (username, group_name) {
        let json_body = serde_json::to_string(&UserGroup {
            username,
            group_name,
        })?;
        let resp: Vec<YewMovieDisplay> = Request::post(&GET_GROUP_MOVIES_URL)
            .header("content-type", "application/json; charset=UTF-8")
            .mode(RequestMode::Cors)
            .body(json_body)
            .send()
            .await?
            .json()
            .await?;
        log::info!("{:?}", &resp);
        Ok(resp)
    } else {
        Err(anyhow!("user or group not set"))
    }
}

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
    username: Option<String>,
    group_name: Option<String>,
    current_movies: HashSet<YewMovieDisplay>,
) -> Result<()> {
    if let (Some(username), Some(group_name)) = (username, group_name) {
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
    } else {
        Err(anyhow!("user or group not set"))
    }
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
