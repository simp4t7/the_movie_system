use crate::utils::get_route_with_auth;
use crate::utils::post_route_with_auth;
use shared_stuff::groups_stuff::AddUser;
use crate::{ADD_USER_URL, CORS_ORIGIN, GET_GROUP_DATA_URL, LEAVE_GROUP_URL};
use anyhow::Result;
use shared_stuff::db_structs::GroupData;
use shared_stuff::MovieDisplay;
use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_get_group_data(group_id: String) -> Result<GroupData> {
    let uri = GET_GROUP_DATA_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let resp = get_route_with_auth(&url).await?;
    log::info!("request_get_all_group_movies resp: {:?}", &resp);
    let group_data: GroupData = resp.json().await?;
    log::info!("request_get_all_group_movies group_data: {:?}", &group_data);
    Ok(group_data)
}

pub async fn request_add_new_user(group_id: String, add_user: String) -> Result<()> {
    let uri = ADD_USER_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let json_body = serde_json::to_string(&AddUser { username: add_user })?;
    //let json_body = String::from("");
    let resp = post_route_with_auth(&url, json_body).await?;
    log::info!("request_add_new_user resp: {:?}", &resp);
    Ok(())
}

pub async fn request_leave_group(group_id: String) -> Result<()> {
    let uri = LEAVE_GROUP_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let json_body = String::from("");
    let resp = post_route_with_auth(&url, json_body).await?;
    log::info!("request_leave_group resp: {:?}", &resp);
    Ok(())
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub group_id: String,
    pub group_data: Option<GroupData>,
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub add_user: String,
}
pub enum GroupMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(GroupData),
    SetAddUser(InputEvent),
    AddUser,
    Leave,
    Error(String),
}

impl Component for Group {
    type Message = GroupMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(GroupMsg::GetGroupData);
        let id = &ctx.props().id;
        Self {
            group_id: id.to_string(),
            group_data: None,
            autocomplete_movies: HashMap::new(),
            add_user: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let id = self.group_id.clone();
        use GroupMsg::*;
        match msg {
            Noop => {}

            GetGroupData => link_clone.send_future(async move {
                let group_data_resp = request_get_group_data(id).await;
                log::info!("group_data_resp: {:?}", &group_data_resp);
                match group_data_resp {
                    Ok(group_data) => GroupMsg::UpdateGroupData(group_data),
                    Err(e) => GroupMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_data) => self.group_data = Some(group_data),

            AddUser => {
                let add_user = self.add_user.clone();
                link_clone.send_future(async move {
                    let resp = request_add_new_user(id, add_user).await;
                    log::info!("{:?}", &resp);
                    GroupMsg::Noop
                })
            }

            SetAddUser(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("add_user value: {:?}", &elem.value());
                    self.add_user = elem.value();
                }
            }

            Leave => ctx.link().send_future(async move {
                let resp = request_leave_group(id).await;
                log::info!("{:?}", &resp);
                GroupMsg::Noop
            }),

            Error(err_msg) => {
                log::info!("{:?}", &err_msg);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
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

impl Group {
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
                    { self.view_add_user_to_group(ctx) }
                    { self.view_leave_group(ctx) }
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
        let system_url = format!("{}/system/{}", CORS_ORIGIN.to_string(), self.group_id);
        html! {
            <div>
                <p>{format!("group data is:")}</p>
                <li>{format!("Name: {}", group_data.group_name)}</li>
                <li>{format!("Members: {:?}", group_data.members)}</li>
                <li>{format!("Date created: {:?}", group_data.date_created)}</li>
                <li>{format!("Movies watched: {:?}", group_data.movies_watched)}</li>
                <li>
                    {"system url: "}
                    <a href= {system_url.clone()}>{system_url}</a>
                </li>
            </div>
        }
    }

    fn view_add_user_to_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Add User"} </h1>
            <input
                class="add_user"
                placeholder="username"
                maxlength=50
                oninput={ctx.link().callback(GroupMsg::SetAddUser)}
            />
            <button
                class="create_group_button"
                onclick={&ctx.link().callback(|_| GroupMsg::AddUser)}>
                { "Add User" }
            </button>
        </div>
        }
    }

    pub fn view_leave_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Leave Group"} </h1>
            <button
                class="create_group_button"
                onclick={ctx.link().callback(|_| GroupMsg::Leave)}>
                { "Leave Group" }
            </button>
        </div>
        }
    }
}
