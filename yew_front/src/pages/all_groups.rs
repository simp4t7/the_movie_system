use crate::auth_requests::post_route_with_auth;
use crate::shared_requests::request_get_group_data;
use crate::{ADD_USER_URL, CORS_ORIGIN, LEAVE_GROUP_URL};
use anyhow::Result;
use shared_stuff::db_structs::{DBGroupStruct, GroupData};
use shared_stuff::group_structs::AddUser;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_add_new_user(group_id: String, add_user: String) -> Result<()> {
    let uri = ADD_USER_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let json_body = serde_json::to_string(&AddUser { username: add_user })?;
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
pub struct AllGroups {
    pub group_id: String,
    pub group_data: Option<GroupData>,
    pub add_user: String,
}
pub enum AllGroupsMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(DBGroupStruct),
    SetAddUser(InputEvent),
    AddUser,
    Leave,
    Error(String),
}

impl Component for AllGroups {
    type Message = AllGroupsMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AllGroupsMsg::GetGroupData);
        let id = &ctx.props().id;
        Self {
            group_id: id.to_string(),
            group_data: None,
            add_user: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let id = self.group_id.clone();
        use AllGroupsMsg::*;
        match msg {
            Noop => {}

            GetGroupData => link_clone.send_future(async move {
                let group_struct_resp = request_get_group_data(id).await;
                log::info!("group_data_resp: {:?}", &group_struct_resp);
                match group_struct_resp {
                    Ok(group_struct) => AllGroupsMsg::UpdateGroupData(group_struct),
                    Err(e) => AllGroupsMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_struct) => {
                self.group_data = Some(group_struct.group_data);
                self.group_id = group_struct.id;
            }

            AddUser => {
                let add_user = self.add_user.clone();
                link_clone.send_future(async move {
                    let resp = request_add_new_user(id, add_user).await;
                    log::info!("{:?}", &resp);
                    AllGroupsMsg::Noop
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
                AllGroupsMsg::Noop
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
