use crate::auth_requests::post_route_with_auth;
use crate::{CREATE_GROUP_URL, GET_ALL_GROUPS_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::group_structs::{GroupForm, GroupInfo, GroupUser};
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_get_all_groups() -> Result<HashSet<GroupInfo>> {
    let json_body = String::from("");
    let resp = post_route_with_auth(&GET_ALL_GROUPS_URL, json_body.clone()).await?;
    let all_groups: HashSet<GroupInfo> = resp.json().await?;
    Ok(all_groups)
}

pub async fn request_create_group(username: String, group_name: String) -> Result<()> {
    let json_body = serde_json::to_string(&GroupForm {
        username,
        group_name,
    })?;
    let resp = post_route_with_auth(&CREATE_GROUP_URL, json_body.clone()).await?;
    log::info!("Create new group resp: {:?}", &resp);
    Ok(())
}

#[derive(Debug, PartialEq, Clone)]
pub struct AllGroups {
    pub username: Option<String>,
    pub add_user: String,
    pub create_group_name: String,
    pub leave_group_name: String,
    pub group_add: String,
    pub current_groups: HashSet<GroupInfo>,
}
pub enum AllGroupsMsg {
    Noop,
    UpdateGroups(HashSet<GroupInfo>),
    CreateGroup,
    CreateGroupName(InputEvent),
    GetAllGroups,
}

impl Component for AllGroups {
    type Message = AllGroupsMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AllGroupsMsg::GetAllGroups);
        let storage = LocalStorage::raw();
        let add_user = String::from("");
        let create_group_name = String::from("");
        let leave_group_name = String::from("");
        let group_add = String::from("");
        let current_groups = HashSet::new();
        let username = storage.get("username").expect("storage problem");
        Self {
            username,
            add_user,
            create_group_name,
            leave_group_name,
            group_add,
            current_groups,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        use AllGroupsMsg::*;
        match msg {
            Noop => {}
            UpdateGroups(groups) => {
                self.current_groups = groups;
            }
            GetAllGroups => link_clone.send_future(async move {
                let groups = request_get_all_groups()
                    .await
                    .expect("problem getting groups");
                AllGroupsMsg::UpdateGroups(groups)
            }),
            CreateGroupName(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("group_name value: {:?}", &elem.value());
                    self.create_group_name = elem.value();
                }
            }
            CreateGroup => {
                //log::info!("making a group, username is: {:?}", &username);
                let group_name = self.create_group_name.clone();
                if let Some(username) = self.username.clone() {
                    ctx.link().send_future(async move {
                        let resp = request_create_group(username, group_name).await;
                        log::info!("{:?}", &resp);
                        AllGroupsMsg::GetAllGroups
                    })
                }
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
         <h1> {"Current Groups"} </h1>
        { self.display_current_groups(ctx) }
        </div> }
    }
}
