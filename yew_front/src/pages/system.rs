use anyhow::Result;
use crate::GET_GROUP_DATA_URL;
use crate::utils::get_route_with_auth;
use serde_json::to_string;
use shared_stuff::db_structs::GroupData;
use shared_stuff::groups_stuff::{AddUser, BasicUsername, GroupForm, GroupInfo, UserGroupsJson};
use std::collections::{HashMap, HashSet};
use web_sys::{HtmlElement, HtmlInputElement};
use shared_stuff::{ImdbQuery, MovieDisplay, YewMovieDisplay};
use yew::prelude::*;

pub async fn request_get_all_group_movies(group_id: String) -> Result<GroupData> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    log::info!("request_get_all_group_movies resp: {:?}", &resp);
    let group_data: GroupData = resp.json().await?;
    log::info!("request_get_all_group_movies group_data: {:?}", &group_data);
    Ok(group_data)
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct System {
    pub group_id: String,
    pub group_data: Option<GroupData>,
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
}
pub enum SystemMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(GroupData),
    Error(String),
}

impl Component for System {
    type Message = SystemMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(SystemMsg::GetGroupData);
        let id = &ctx.props().id;
        Self {
            group_id: id.to_string(),
            group_data: None,
            autocomplete_movies: HashMap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let id = self.group_id.clone();
        use SystemMsg::*;
        match msg {
            Noop => {},

            GetGroupData => link_clone.send_future(async move {
                let group_data_resp = request_get_all_group_movies(id).await;
                log::info!("group_data_resp: {:?}", &group_data_resp);
                match group_data_resp {
                    Ok(group_data) => SystemMsg::UpdateGroupData(group_data),
                    Err(e) => SystemMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_data) => self.group_data = Some(group_data),

            Error(err_msg) => {
                log::info!("{:?}", &err_msg);
            },
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            { self.view_group_id(ctx) }
            { self.user_customized_view(ctx) }
            </div>
        }

    }
}


impl System {
    fn view_group_id(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }

    fn user_customized_view(&self, ctx: &Context<Self>) -> Html {
        match &self.group_data {
            Some(group_data) => {
                html! {
                    <div>
                    { self.view_group_data(ctx, &group_data) }
                    </div>
                }
            },
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
}
