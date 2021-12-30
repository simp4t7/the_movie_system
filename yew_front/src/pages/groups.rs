use crate::utils::auth_flow;
use crate::GlobalState;
use anyhow::Result;
use gloo_storage::Storage;
use lazy_static::lazy_static;
use load_dotenv::load_dotenv;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::groups_stuff::AddUser;
use shared_stuff::groups_stuff::BasicUsername;
use shared_stuff::groups_stuff::UserGroupsJson;

use gloo_storage::LocalStorage;
use shared_stuff::groups_stuff::GroupForm;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

lazy_static! {
    pub static ref ROOT_URL: &'static str = {
        load_dotenv!();
        env!("ROOT_URL")
    };
    pub static ref CREATE_GROUP_URL: String = format!("{}/create_group", *ROOT_URL);
    pub static ref LEAVE_GROUP_URL: String = format!("{}/leave_group", *ROOT_URL);
    pub static ref ADD_USER_URL: String = format!("{}/add_user", *ROOT_URL);
    pub static ref GET_ALL_GROUPS_URL: String = format!("{}/get_groups", *ROOT_URL);
}

pub async fn new_group_request(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = Request::post(&CREATE_GROUP_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    log::info!("{:?}", &resp);
    Ok(())
}
pub async fn leave_group_request(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = Request::post(&LEAVE_GROUP_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    log::info!("{:?}", &resp);
    Ok(())
}

pub async fn add_user_request(
    username: String,
    add_user: String,
    group_name: String,
) -> Result<()> {
    let json_body = serde_json::to_string(&AddUser {
        username,
        add_user,
        group_name,
    })?;
    let resp = Request::post(&ADD_USER_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    log::info!("{:?}", &resp);
    Ok(())
}

pub async fn get_all_groups(username: String) -> Result<Vec<String>> {
    let json_body = serde_json::to_string(&BasicUsername { username })?;
    let resp = Request::post(&GET_ALL_GROUPS_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    let groups: UserGroupsJson = resp.json().await?;
    log::info!("{:?}", &groups);
    Ok(groups.groups)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    add_user: String,
    create_group_name: String,
    leave_group_name: String,
    group_add: String,
}
pub enum GroupsMsg {
    GroupAdd(InputEvent),
    UpdateGroups(Vec<String>),
    CreateGroup,
    LeaveGroup,
    CreateGroupName(InputEvent),
    LeaveGroupName(InputEvent),
    AddUser(InputEvent),
    AddNewUser,
    GetAllGroups,
}

impl Component for Groups {
    type Message = GroupsMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(GroupsMsg::GetAllGroups);
        let add_user = String::from("");
        let create_group_name = String::from("");
        let leave_group_name = String::from("");
        let group_add = String::from("");
        Self {
            add_user,
            create_group_name,
            leave_group_name,
            group_add,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let storage = LocalStorage::raw();
        let username_option = storage.get("username").expect("problem getting username");
        let username = username_option.expect("username is empty?");
        use GroupsMsg::*;
        match msg {
            UpdateGroups(groups_vec) => {
                storage
                    .set(
                        "all_groups",
                        serde_json::to_string(&groups_vec)
                            .expect("serde error")
                            .as_str(),
                    )
                    .expect("storage problem");
            }
            GetAllGroups => link_clone.send_future(async move {
                let groups = get_all_groups(username)
                    .await
                    .expect("problem getting groups");
                GroupsMsg::UpdateGroups(groups)
            }),
            GroupAdd(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.group_add = elem.value();
                }
            }
            LeaveGroup => {
                let group_name = self.leave_group_name.clone();
                log::info!("username: {:?}, group_name: {:?}", &username, &group_name);
                spawn_local(async move {
                    let resp = leave_group_request(username, group_name).await;
                    log::info!("{:?}", &resp);
                })
            }
            CreateGroupName(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("group_name value: {:?}", &elem.value());
                    self.create_group_name = elem.value();
                }
            }
            LeaveGroupName(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("group_name value: {:?}", &elem.value());
                    self.leave_group_name = elem.value();
                }
            }
            AddNewUser => {
                let add_user = self.add_user.clone();
                let group_add = self.group_add.clone();
                spawn_local(async move {
                    let resp = add_user_request(username, add_user, group_add).await;
                    log::info!("{:?}", &resp);
                })
            }
            AddUser(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("add_user value: {:?}", &elem.value());
                    self.add_user = elem.value();
                }
            }
            CreateGroup => {
                log::info!("making a group, username is: {:?}", &username);
                let group_name = self.create_group_name.clone();
                spawn_local(async move {
                    let resp = new_group_request(username, group_name).await;
                    log::info!("{:?}", &resp);
                })
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("groups stuff: {:?}", self);
        html! {
        <div>
        { self.create_group(ctx) }
        { self.leave_group(ctx) }
        { self.add_user_to_group(ctx) }
        </div> }
    }
}
