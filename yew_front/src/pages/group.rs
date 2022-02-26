use crate::auth_requests::post_route_with_auth;
use crate::shared_requests::request_get_group_data;
use crate::{ADD_USER_URL, LEAVE_GROUP_URL};
use anyhow::Result;
use reqwasm::http::Response;
use shared_stuff::auth_structs::ErrorMessage;
// use gloo_storage::Result;
use shared_stuff::db_structs::{DBGroupStruct, GroupData};
use shared_stuff::group_structs::AddUser;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_add_new_user(group_id: String, add_user: String) -> Result<Response> {
    let uri = ADD_USER_URL.to_string();
    let url = format!("{}/{}", uri, group_id);
    let json_body = serde_json::to_string(&AddUser { username: add_user })?;
    let resp = post_route_with_auth(&url, json_body).await;
    log::info!("request_add_new_user resp: {:?}", &resp);
    resp
}

pub async fn get_add_new_user_resp_status(group_id: String, add_user: String) -> Result<String> {
    let resp_result = request_add_new_user(group_id, add_user).await;
    match resp_result {
        Ok(resp) => {
            match resp.status() {
                200 => Ok(String::from("ok")),
                _ => {
                    let resp_body: ErrorMessage = resp.json().await?;
                    Ok(resp_body.message)
                }
            }
        },
        Err(e) => Ok(e.to_string()),
    }
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
    pub add_user: String,
    pub add_user_status: String,
}
pub enum GroupMsg {
    Noop,
    GetGroupData,
    UpdateGroupData(DBGroupStruct),
    SetAddUser(InputEvent),
    UpdateAddUserStatus(String),
    AddUser,
    Leave,
    Error(String),
}

impl Component for Group {
    type Message = GroupMsg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(GroupMsg::GetGroupData);
        Self {
            group_id: ctx.props().id.clone(),
            group_data: None,
            add_user: String::from(""),
            add_user_status: String::from("ok"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        let group_id = self.group_id.clone();
        use GroupMsg::*;
        match msg {
            Noop => {}

            GetGroupData => link_clone.send_future(async move {
                let group_struct_resp = request_get_group_data(group_id).await;
                match group_struct_resp {
                    Ok(group_struct) => GroupMsg::UpdateGroupData(group_struct),
                    Err(e) => GroupMsg::Error(e.to_string()),
                }
            }),

            UpdateGroupData(group_struct) => {
                self.group_data = Some(group_struct.group_data);
                self.group_id = group_struct.id;
            }

            AddUser => {
                let add_user = self.add_user.clone();
                link_clone.send_future(async move {
                    let add_user_result = get_add_new_user_resp_status(group_id, add_user).await;
                    match add_user_result {
                        Ok(text) => UpdateAddUserStatus(text),
                        _ => UpdateAddUserStatus(String::from("bad request")),
                    }
                })
            }


            UpdateAddUserStatus(add_user_status_text) => {
                self.add_user_status = add_user_status_text;
            }

            SetAddUser(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.add_user = elem.value();
                }
            }

            Leave => ctx.link().send_future(async move {
                let _resp = request_leave_group(group_id).await;
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
