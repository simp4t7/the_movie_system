use crate::auth_requests::{post_route_with_auth, get_route_with_auth};
use crate::{CREATE_GROUP_URL, GET_ALL_GROUPS_URL, GET_USER_PROFILE};
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::group_structs::{GroupForm, GroupInfo, UserProfile};
use std::collections::HashSet;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_get_user_profile(username: String) -> Result<UserProfile> {
    let url = format!("{}/{}", GET_USER_PROFILE.to_string(), username);
    log::info!("get_user_profile request url: {:?}", &url);
    let resp = get_route_with_auth(&url).await?;
    log::info!("get_user_profile request resp: {:?}", &resp);
    let user_profile: UserProfile = resp.json().await?;
    log::info!("get_user_profile request resp user_profile: {:?}", &user_profile);
    Ok(user_profile)
}

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
    pub user_profile: Option<UserProfile>,
    pub create_group_name: String,
    pub all_groups: HashSet<GroupInfo>,
}

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub username: String,
}

pub enum UserMsg {
    Noop,
    GetUserProfile,
    CreateGroup,
    CreateGroupName(InputEvent),
    GetAllGroups,
    UpdateGroups(HashSet<GroupInfo>),
    UpdateUserProfile(UserProfile),
}

impl Component for User {
    type Message = UserMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(UserMsg::GetUserProfile);
        let create_group_name = String::from("");
        Self {
            username: ctx.props().username.clone(),
            user_profile: None,
            create_group_name,
            all_groups: HashSet::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let username = self.username.clone();
        use UserMsg::*;
        match msg {
            Noop => {}
            GetUserProfile => {
                link_clone.send_future(async move {
                    let user_profile_result = request_get_user_profile(username).await;
                    match user_profile_result {
                        Ok(user_profile) => UserMsg::UpdateUserProfile(user_profile),
                        _ => UserMsg::Noop,
                    }
                })
            }
            GetAllGroups => {
                link_clone.send_future(async move {
                    let groups_result = request_get_all_groups().await;
                    match groups_result {
                        Ok(groups) => UserMsg::UpdateGroups(groups),
                        _ => UserMsg::Noop,
                    }
                })
            },
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
            UpdateUserProfile(user_profile) => {
                self.user_profile = Some(user_profile);
                ctx.link().send_future(async move {
                    UserMsg::GetAllGroups
                });
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
        { self.user_customized_view(ctx) }
        </div> }
    }
}
