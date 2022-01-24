use crate::utils::get_route_with_auth;
use crate::utils::post_route_with_auth;
use crate::{ADD_USER_URL, CREATE_GROUP_URL, GET_ALL_GROUPS_URL, LEAVE_GROUP_URL, GET_GROUP_DATA_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::db_structs::GroupData;
use shared_stuff::groups_stuff::{AddUser, BasicUsername, GroupForm, GroupInfo, UserGroupsJson};
use std::collections::HashSet;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;


pub async fn request_get_all_group_movies(group_id: String) -> Result<GroupData> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{:?}/{:?}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    let group_data: GroupData = resp.json().await?;
    Ok(group_data)
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub group_data: GroupData,
    pub group_id: String,
}
pub enum GroupMsg {
    Noop,
    GetGroupData,
}

impl Component for Group {
    type Message = GroupMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(GroupMsg::GetGroupData);
        let id = &ctx.props().id;
        Self {
            group_data: GroupData::new_empty(),
            group_id: id.to_string(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        // let storage = LocalStorage::raw();
        // let username_option = storage.get("username").expect("problem getting username");
        // let username = username_option.expect("username is empty?");
        let id = self.group_id.clone();
        use GroupMsg::*;
        match msg {
            Noop => {}
            GetGroupData => link_clone.send_future(async move {
                let _groups = request_get_all_group_movies(id)
                    .await
                    .expect("problem getting groups");
                GroupMsg::Noop
            }),
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
                
        }
    }
}

/*
use shared_stuff::{ImdbQuery, MovieDisplay, YewMovieDisplay, db_structs::GroupData};
use std::collections::{HashMap, HashSet};
use crate::GET_GROUP_DATA_URL;
use yew::prelude::*;
use anyhow::Result;
use crate::utils::get_route_with_auth;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

pub enum GroupMsg {
    Noop,
    GetGroupData,
    // Error(String),
}

pub struct Group {
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub group_data: GroupData,
    pub group_id: String,
}

impl Component for Group {
    type Message = GroupMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("creating group page for group_id: {}", &ctx.props().id);
        let id_ptr = &ctx.props().id.to_string();
        ctx.link().send_message(GroupMsg::GetGroupData);

        Self {
            autocomplete_movies: HashMap::new(),
            group_data: GroupData::new_empty(),
            group_id: id_ptr.to_owned(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GroupMsg::*;
        let link_clone = ctx.link().clone();
        let group_id = &ctx.props().id.clone();

        match msg {
            GetGroupData => link_clone.send_future(async move {
                let g = request_group_movie_list(self.group_id).await; 
                GroupMsg::Noop
            }),
            Noop => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }
}

pub async fn request_group_movie_list(group_id: String) -> Result<GroupData> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{:?}/{:?}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    let group_data: GroupData = resp.json().await?;
    Ok(group_data)
}
*/
