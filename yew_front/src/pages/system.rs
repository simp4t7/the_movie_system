use crate::{SAVE_GROUP_MOVIES_URL, SEARCH_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode};
use shared_stuff::db_structs::SystemState;
use shared_stuff::groups_stuff::GroupMoviesForm;

use crate::utils::get_route_with_auth;
use crate::GET_GROUP_DATA_URL;
use shared_stuff::db_structs::GroupData;
use shared_stuff::{ImdbQuery, MovieDisplay, YewMovieDisplay};
use std::collections::{HashMap, HashSet};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub username: String,
    pub group_id: String,
    pub group_data: Option<GroupData>,
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub current_movies: HashSet<YewMovieDisplay>,
    pub members_vec: Vec<String>,
}
pub enum SystemMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(GroupData),
    Error(String),
    SaveMovies,
    DeleteEntry(YewMovieDisplay),
    DeleteEntryChangeTurn(YewMovieDisplay),
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddMovie(MouseEvent),
    SetReady,
    UnsetReady,
}

impl Component for System {
    type Message = SystemMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(SystemMsg::GetGroupData);
        let storage = LocalStorage::raw();
        let id = &ctx.props().id;
        let mut username = String::from("");
        let current_movies = HashSet::new();
        let members_vec = Vec::new();
        if let Some(user) = storage.get("username").expect("storage error") {
            username = user;
        }
        Self {
            username,
            group_id: id.to_string(),
            group_data: None,
            autocomplete_movies: HashMap::new(),
            current_movies,
            members_vec,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let id = self.group_id.clone();
        if let Some(data) = self.group_data.clone() {
            self.current_movies = data.current_movies;
        }
        use SystemMsg::*;
        match msg {
            Noop => {}
            SetReady => {
                if let Some(mut data) = self.group_data.clone() {
                    data.ready_status.insert(self.username.clone(), true);
                    if data
                        .members
                        .iter()
                        .all(|member| data.ready_status.get(member) == Some(&true))
                    {
                        data.system_state = SystemState::SystemStarted;
                        let current_turn = self.members_vec.remove(0);
                        data.turn = current_turn.clone();
                        self.members_vec.push(current_turn);
                    }
                    self.group_data = Some(data.clone());
                    let cloned_data = data.clone();
                    let cloned_id = self.group_id.clone();
                    link_clone.send_future(async move {
                        let resp = request_update_group_data(cloned_id, cloned_data).await;
                        log::info!("resp is: {:?}", &resp);
                        SystemMsg::Noop
                    })
                }
            }
            UnsetReady => {
                if let Some(mut data) = self.group_data.clone() {
                    data.ready_status.insert(self.username.clone(), false);
                    self.group_data = Some(data.clone());
                    let cloned_data = data.clone();
                    let cloned_id = self.group_id.clone();
                    link_clone.send_future(async move {
                        let resp = request_update_group_data(cloned_id, cloned_data).await;
                        log::info!("resp is: {:?}", &resp);
                        SystemMsg::Noop
                    })
                }
            }
            SaveMovies => {
                log::info!("inside save movies");
                let group_id = self.group_id.clone();
                let data = self.group_data.clone();
                if let Some(mut group_data) = data {
                    log::info!("inside SaveMovies, group_data: {:?}", &group_data);
                    group_data.current_movies = self.current_movies.clone();
                    link_clone.send_future(async move {
                        let resp = request_update_group_data(group_id, group_data).await;
                        log::info!("resp is: {:?}", &resp);
                        SystemMsg::Noop
                    })
                }
            }

            DeleteEntry(movie) => {
                self.current_movies.remove(&movie);
                let group_id = self.group_id.clone();
                if let Some(mut data) = self.group_data.clone() {
                    data.current_movies = self.current_movies.clone();
                    self.group_data = Some(data.clone());
                    link_clone.send_future(async move {
                        let resp = request_update_group_data(group_id, data.clone()).await;
                        log::info!("resp is: {:?}", &resp);
                        SystemMsg::Noop
                    })
                }
            }

            DeleteEntryChangeTurn(movie) => {
                self.current_movies.remove(&movie);
                let group_id = self.group_id.clone();
                if let Some(mut data) = self.group_data.clone() {
                    data.current_movies = self.current_movies.clone();
                    let current_turn = self.members_vec.remove(0);
                    data.turn = current_turn.clone();
                    self.members_vec.push(current_turn);
                    self.group_data = Some(data.clone());
                    if self.current_movies.len() == 1 {
                        data.system_state = SystemState::Finished;
                    }
                    link_clone.send_future(async move {
                        let resp = request_update_group_data(group_id, data.clone()).await;
                        log::info!("resp is: {:?}", &resp);
                        SystemMsg::Noop
                    })
                }
            }
            AddMovie(movie) => {
                if let Some(elem) = movie.target_dyn_into::<HtmlElement>() {
                    log::info!("inside AddMovie");
                    log::info!("checking current movies: {:?}", &self.autocomplete_movies);
                    log::info!("current element: {:?}", &elem.title());
                    let lookup_title = &elem.id();
                    let movie = self
                        .autocomplete_movies
                        .get(lookup_title)
                        .expect("not here");
                    log::info!("movie is: {:?}", &movie);
                    self.current_movies
                        .insert(movie.clone().into_yew_display(self.username.clone()));
                    if let Some(mut data) = self.group_data.clone() {
                        data.current_movies
                            .insert(movie.clone().into_yew_display(self.username.clone()));
                        self.group_data = Some(data);
                    }
                    log::info!("current_movies: {:?}", &self.current_movies);

                    //let movie_set = self.added_movies.values().cloned().collect::<HashSet<_>>();
                    //let json_movies = serde_json::to_string(&movie_set).expect("json issue");
                    //storage
                    //.set("added_movies", &json_movies)
                    //.expect("storage issue");
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

                            match request_get_search_results(&SEARCH_URL, query).await {
                                Ok(resp) => SystemMsg::UpdateAutocomplete(resp),
                                Err(err_msg) => {
                                    link_clone.clone().send_message(UpdateAutocomplete(vec![]));
                                    SystemMsg::Error(err_msg.to_string())
                                }
                            }
                        } else {
                            SystemMsg::Noop
                        }
                    });
                }
            }

            SystemMsg::UpdateAutocomplete(movies) => {
                log::info!("{:?}", &movies);
                self.autocomplete_movies.clear();
                for i in movies {
                    self.autocomplete_movies.insert(i.movie_id.clone(), i);
                }

                //self.autocomplete_movies = movies;
            }

            GetGroupData => link_clone.send_future(async move {
                let group_data_resp = request_get_all_group_movies(id).await;
                log::info!("group_data_resp: {:?}", &group_data_resp);
                match group_data_resp {
                    Ok(group_data) => SystemMsg::UpdateGroupData(group_data),
                    Err(e) => SystemMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_data) => {
                self.group_data = Some(group_data.clone());
                self.current_movies = group_data.current_movies;
                self.members_vec = group_data.members.iter().cloned().collect::<Vec<String>>();
            }

            Error(err_msg) => {
                log::info!("{:?}", &err_msg);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            { self.ready_status_buttons(ctx) }
            { self.view_group_id(ctx) }
            { self.user_customized_view(ctx) }
            { self.search_bar(ctx) }
            { self.add_stuff(ctx) }
            </div>
        }
    }
}

use shared_stuff::db_structs::DBGroupStruct;

pub async fn request_update_group_data(group_id: String, group_data: GroupData) -> Result<()> {
    let db_group = DBGroupStruct {
        id: group_id,
        group_data,
    };
    let serialized_db_group = serde_json::to_string(&db_group).expect("serialization error");
    let _resp = Request::post(&SAVE_GROUP_MOVIES_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(serialized_db_group)
        .send()
        .await?;
    Ok(())
}

//pub async fn request_save_movies_request(
//username: String,
//group_id: String,
//current_movies: HashSet<YewMovieDisplay>,
//) -> Result<()> {
//let json_body = serde_json::to_string(&GroupMoviesForm {
//username,
//group_id,
//current_movies,
//})?;
//let _resp = Request::post(&SAVE_GROUP_MOVIES_URL)
//.header("content-type", "application/json; charset=UTF-8")
//.mode(RequestMode::Cors)
//.body(json_body)
//.send()
//.await?;
//Ok(())
//}

pub async fn request_get_all_group_movies(group_id: String) -> Result<GroupData> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    log::info!("request_get_all_group_movies resp: {:?}", &resp);
    let group_data: GroupData = resp.json().await?;
    log::info!("request_get_all_group_movies group_data: {:?}", &group_data);
    Ok(group_data)
}

pub async fn request_get_search_results(url: &str, body: ImdbQuery) -> Result<Vec<MovieDisplay>> {
    if !body.query.is_empty() {
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
    } else {
        Ok(vec![])
    }
}
