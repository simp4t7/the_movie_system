use crate::SEARCH_URL;
use crate::UPDATE_GROUP_DATA_URL;
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode};

use crate::shared_requests::request_get_group_data;
use shared_stuff::db_structs::GroupData;
use shared_stuff::imdb_structs::ImdbQuery;
use shared_stuff::shared_structs::{MovieDisplay, SystemState, YewMovieDisplay};
use std::collections::{HashSet, VecDeque};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub username: String,
    pub group_id: String,
    pub group_data: GroupData,
    pub autocomplete_movies: HashSet<MovieDisplay>,
    pub current_movies: HashSet<YewMovieDisplay>,
    pub loaded: bool,
}
pub enum SystemMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(DBGroupStruct),
    Error(String),
    SaveMovies,
    DeleteEntry(YewMovieDisplay),
    DeleteEntryChangeTurn(YewMovieDisplay),
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddMovie(MovieDisplay),
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
        if let Some(user) = storage.get("username").expect("storage error") {
            username = user;
        }
        Self {
            username,
            group_id: id.to_string(),
            group_data: GroupData::new_empty(),
            autocomplete_movies: HashSet::new(),
            current_movies,
            loaded: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let id = self.group_id.clone();
        let storage = LocalStorage::raw();
        let username = storage
            .get("username")
            .expect("storage error")
            .expect("ummm");
        self.current_movies = self.group_data.current_movies.clone();
        use SystemMsg::*;
        match msg {
            Noop => {}
            SetReady => {
                if let Some(user_status) = self.group_data.members.get_mut(&username) {
                    user_status.ready_status = true;
                } else {
                    log::info!("some problem here");
                };
                if self
                    .group_data
                    .members
                    .values()
                    .all(|user_status| user_status.ready_status == true)
                {
                    self.group_data.system_state = SystemState::SystemStarted;
                    self.group_data.system_order = self
                        .group_data
                        .members
                        .keys()
                        .cloned()
                        .collect::<VecDeque<String>>();
                    let current_turn = self
                        .group_data
                        .system_order
                        .pop_front()
                        .expect("no members?");
                    self.group_data.turn = current_turn.clone();
                    self.group_data.system_order.push_back(current_turn);
                }
                let cloned_data = self.group_data.clone();
                let cloned_id = self.group_id.clone();
                link_clone.send_future(async move {
                    let resp = request_update_group_data(cloned_id, cloned_data).await;
                    log::info!("resp is: {:?}", &resp);
                    SystemMsg::Noop
                })
            }
            UnsetReady => {
                if let Some(user_status) = self.group_data.members.get_mut(&username) {
                    user_status.ready_status = false;
                } else {
                    log::info!("some problem here");
                };
                let cloned_data = self.group_data.clone();
                let cloned_id = self.group_id.clone();
                link_clone.send_future(async move {
                    let resp = request_update_group_data(cloned_id, cloned_data).await;
                    log::info!("resp is: {:?}", &resp);
                    SystemMsg::Noop
                })
            }
            SaveMovies => {
                log::info!("inside save movies");
                self.group_data.current_movies = self.current_movies.clone();

                let cloned_data = self.group_data.clone();
                let cloned_id = self.group_id.clone();
                link_clone.send_future(async move {
                    let resp = request_update_group_data(cloned_id, cloned_data).await;
                    log::info!("resp is: {:?}", &resp);
                    SystemMsg::Noop
                })
            }

            DeleteEntry(movie) => {
                self.current_movies.remove(&movie);
                self.group_data.current_movies = self.current_movies.clone();
                let cloned_data = self.group_data.clone();
                let cloned_id = self.group_id.clone();
                link_clone.send_future(async move {
                    let resp = request_update_group_data(cloned_id, cloned_data).await;
                    log::info!("resp is: {:?}", &resp);
                    SystemMsg::Noop
                })
            }

            DeleteEntryChangeTurn(movie) => {
                self.current_movies.remove(&movie);
                self.group_data.current_movies = self.current_movies.clone();
                let current_turn = self
                    .group_data
                    .system_order
                    .pop_front()
                    .expect("members empty?");
                self.group_data.turn = current_turn.clone();
                self.group_data.system_order.push_back(current_turn);
                if self.current_movies.len() == 1 {
                    self.group_data.system_state = SystemState::Finished;
                }
                link_clone.send_message(SystemMsg::UpdateGroupData(
                    self.group_data.clone().into_db_group_struct(&self.group_id),
                ));
                let cloned_data = self.group_data.clone();
                let cloned_id = self.group_id.clone();

                link_clone.send_future(async move {
                    let resp = request_update_group_data(cloned_id, cloned_data).await;
                    log::info!("resp is: {:?}", &resp);
                    SystemMsg::Noop
                })
            }
            AddMovie(movie) => {
                self.current_movies
                    .insert(movie.clone().into_yew_display(self.username.clone()));
                self.group_data
                    .current_movies
                    .insert(movie.clone().into_yew_display(self.username.clone()));
                log::info!("current_movies: {:?}", &self.current_movies);
            }
            QueryAutocomplete(text) => {
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
                    self.autocomplete_movies.insert(i);
                }
            }

            GetGroupData => link_clone.send_future(async move {
                let group_data_resp = request_get_group_data(id).await;
                log::info!("group_data_resp: {:?}", &group_data_resp);
                match group_data_resp {
                    Ok(group_data) => SystemMsg::UpdateGroupData(group_data),
                    Err(e) => SystemMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_struct) => {
                self.group_data = group_struct.group_data.clone();
                self.current_movies = group_struct.group_data.current_movies;
                self.group_id = group_struct.id;
                self.loaded = true;
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
            { self.full_search_html(ctx) }
            //{ self.display_current_members(ctx) }
            //{ self.search_bar(ctx) }
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
    let _resp = Request::post(&UPDATE_GROUP_DATA_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(serialized_db_group)
        .send()
        .await?;
    Ok(())
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
