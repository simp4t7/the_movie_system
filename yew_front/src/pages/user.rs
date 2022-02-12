use crate::auth_requests::post_route_with_auth;
use crate::{CREATE_GROUP_URL, GET_ALL_GROUPS_URL};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::group_structs::{GroupForm, GroupInfo};
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
pub struct User {
    pub username: String,
    pub create_group_name: String,
    pub all_groups: HashSet<GroupInfo>,
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub username: String,
}

pub enum UserMsg {
    Noop,
    CreateGroup,
    CreateGroupName(InputEvent),
    GetAllGroups,
    UpdateGroups(HashSet<GroupInfo>),
}

impl Component for User {
    type Message = UserMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(UserMsg::GetAllGroups);
        let create_group_name = String::from("");
        Self {
            username: ctx.props().username.clone(),
            create_group_name,
            all_groups: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        use UserMsg::*;
        match msg {
            Noop => {}
            GetAllGroups => link_clone.send_future(async move {
                let groups = request_get_all_groups()
                    .await
                    .expect("problem getting groups");
                UserMsg::UpdateGroups(groups)
            }),
            CreateGroupName(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("group_name value: {:?}", &elem.value());
                    self.create_group_name = elem.value();
                }
            }
            CreateGroup => {
                let group_name = self.create_group_name.clone();
                if let username = self.username.clone() {
                    ctx.link().send_future(async move {
                        let resp = request_create_group(username, group_name).await;
                        log::info!("{:?}", &resp);
                        UserMsg::GetAllGroups
                    })
                }
            }
            UpdateGroups(groups) => {
                self.all_groups = groups;
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
        { self.user_info(ctx) }
        { self.create_group(ctx) }
         <h1> {"Current Groups"} </h1>
        { self.display_all_groups(ctx) }
        </div> }
    }
}
