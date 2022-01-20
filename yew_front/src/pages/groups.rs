use crate::utils::post_route_with_auth;
use crate::{ADD_USER_URL, CREATE_GROUP_URL, GET_ALL_GROUPS_URL, LEAVE_GROUP_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use reqwasm::http::{Request, RequestMode};
use shared_stuff::groups_stuff::{AddUser, BasicUsername, GroupForm, UserGroupsJson};
use std::collections::HashSet;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

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
    new_member: String,
    group_name: String,
) -> Result<()> {
    let json_body = serde_json::to_string(&AddUser {
        username,
        new_member,
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
    let resp = post_route_with_auth(&GET_ALL_GROUPS_URL, json_body.clone()).await?;
    let groups: UserGroupsJson = resp.json().await?;
    Ok(groups.groups)
}

pub async fn new_group_request(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = post_route_with_auth(&CREATE_GROUP_URL, json_body.clone()).await?;

    /*
    let resp = Request::post(&CREATE_GROUP_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
        */
    log::info!("Create new group resp: {:?}", &resp);
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    pub add_user: String,
    pub create_group_name: String,
    pub leave_group_name: String,
    pub group_add: String,
    pub current_groups: HashSet<String>,
}
pub enum GroupsMsg {
    Noop,
    GroupAdd(InputEvent),
    UpdateGroups(Vec<String>),
    CreateGroup,
    LeaveGroup,
    CreateGroupName(InputEvent),
    LeaveGroupName(InputEvent),
    AddUser(InputEvent),
    SetCurrentGroup(MouseEvent),
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
        let current_groups = HashSet::new();
        Self {
            add_user,
            create_group_name,
            leave_group_name,
            group_add,
            current_groups,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let storage = LocalStorage::raw();
        let username_option = storage.get("username").expect("problem getting username");
        let username = username_option.expect("username is empty?");
        log::info!("username is: {:?}", &username);
        use GroupsMsg::*;
        match msg {
            Noop => {}
            SetCurrentGroup(group) => {
                if let Some(elem) = group.target_dyn_into::<HtmlElement>() {
                    log::info!("inside set current group");
                    storage
                        .set("current_group", &elem.inner_text())
                        .expect("storage issue");
                }
            }
            UpdateGroups(groups_vec) => {
                storage
                    .set(
                        "all_groups",
                        serde_json::to_string(&groups_vec)
                            .expect("serde error")
                            .as_str(),
                    )
                    .expect("storage problem");
                let new_hash: HashSet<String> = HashSet::from_iter(groups_vec.iter().cloned());
                self.current_groups = new_hash;
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
                ctx.link().send_future(async move {
                    let resp = leave_group_request(username, group_name).await;
                    log::info!("{:?}", &resp);
                    GroupsMsg::Noop
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
                ctx.link().send_future(async move {
                    let resp = add_user_request(username, add_user, group_add).await;
                    log::info!("{:?}", &resp);
                    GroupsMsg::Noop
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
                ctx.link().send_future(async move {
                    let resp = new_group_request(username, group_name).await;
                    log::info!("{:?}", &resp);
                    GroupsMsg::GetAllGroups
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
         <h1> {"Current Groups"} </h1>
        { self.display_current_groups(ctx) }
        </div> }
    }
}
