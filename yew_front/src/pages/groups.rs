use crate::utils::post_route_with_auth;
use crate::{CREATE_GROUP_URL, GET_ALL_GROUPS_URL, LEAVE_GROUP_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::groups_stuff::{GroupForm, GroupInfo, UserGroupsJson};
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_leave_group(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = post_route_with_auth(&LEAVE_GROUP_URL, json_body.clone()).await?;
    log::info!("leave group resp: {:?}", &resp);
    Ok(())
}

//pub async fn request_add_user(
//username: String,
//new_member: String,
//group_name: String,
//) -> Result<()> {
//let json_body = serde_json::to_string(&AddUser {
//username,
//new_member,
//group_name,
//})?;
//let resp = post_route_with_auth(&ADD_USER_URL, json_body.clone()).await?;
//log::info!("add user resp: {:?}", &resp);
//Ok(())
//}

pub async fn request_get_all_groups() -> Result<HashSet<GroupInfo>> {
    //let json_body = serde_json::to_string(&Add { username })?;
    let json_body = String::from("");
    let resp = post_route_with_auth(&GET_ALL_GROUPS_URL, json_body.clone()).await?;
    let groups: UserGroupsJson = resp.json().await?;
    Ok(groups.groups)
}

pub async fn request_new_group(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = post_route_with_auth(&CREATE_GROUP_URL, json_body.clone()).await?;
    log::info!("Create new group resp: {:?}", &resp);
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    pub add_user: String,
    pub create_group_name: String,
    pub leave_group_name: String,
    pub group_add: String,
    pub current_groups: HashSet<GroupInfo>,
}
pub enum GroupsMsg {
    Noop,
    UpdateGroups(HashSet<GroupInfo>),
    CreateGroup,
    CreateGroupName(InputEvent),
    // SetCurrentGroup(MouseEvent),
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
            UpdateGroups(groups_vec) => {
                storage
                    .set(
                        "all_groups",
                        serde_json::to_string(&groups_vec)
                            .expect("serde error")
                            .as_str(),
                    )
                    .expect("storage problem");
                let new_hash: HashSet<GroupInfo> = HashSet::from_iter(groups_vec.iter().cloned());
                self.current_groups = new_hash;
            }
            GetAllGroups => link_clone.send_future(async move {
                let groups = request_get_all_groups()
                    .await
                    .expect("problem getting groups");
                GroupsMsg::UpdateGroups(groups)
            }),
            CreateGroupName(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("group_name value: {:?}", &elem.value());
                    self.create_group_name = elem.value();
                }
            }
            CreateGroup => {
                log::info!("making a group, username is: {:?}", &username);
                let group_name = self.create_group_name.clone();
                ctx.link().send_future(async move {
                    let resp = request_new_group(username, group_name).await;
                    log::info!("{:?}", &resp);
                    GroupsMsg::GetAllGroups
                })
            } /*
              SetCurrentGroup(group) => {
                  if let Some(elem) = group.target_dyn_into::<HtmlElement>() {
                      log::info!("inside set current group");
                      storage
                          .set("current_group", &elem.inner_text())
                          .expect("storage issue");
                  }
              }
              */
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
         <h1> {"Current Groups"} </h1>
        { self.display_current_groups(ctx) }
        </div> }
    }
}
