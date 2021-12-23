use crate::utils::auth_flow;
use anyhow::Result;
use lazy_static::lazy_static;
use load_dotenv::load_dotenv;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::groups_stuff::AddUser;
use shared_stuff::groups_stuff::BasicUsername;
use shared_stuff::groups_stuff::GroupForm;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    username: String,
    add_user: String,
    create_group_name: String,
    leave_group_name: String,
}
pub enum GroupsMsg {
    CreateGroup,
    LeaveGroup,
    UpdateUsername,
    CreateGroupName(InputEvent),
    LeaveGroupName(InputEvent),
    SetUsername(String),
    AddUser(InputEvent),
    AddNewUser,
}

lazy_static! {
    pub static ref ROOT_URL: &'static str = {
        load_dotenv!();
        env!("ROOT_URL")
    };
    pub static ref CREATE_GROUP_URL: String = format!("{}/create_group", *ROOT_URL);
    pub static ref LEAVE_GROUP_URL: String = format!("{}/leave_group", *ROOT_URL);
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

pub async fn add_user_request(username: String, add_user: String) -> Result<()> {
    let json_body = serde_json::to_string(&AddUser { username, add_user })?;
    let resp = Request::post(&CREATE_GROUP_URL)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(json_body)
        .send()
        .await?;
    log::info!("{:?}", &resp);
    Ok(())
}

impl Component for Groups {
    type Message = GroupsMsg;
    type Properties = ();
    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(GroupsMsg::UpdateUsername);
        let username = String::from("");
        let add_user = String::from("");
        let create_group_name = String::from("");
        let leave_group_name = String::from("");
        Self {
            username,
            add_user,
            create_group_name,
            leave_group_name,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GroupsMsg::*;
        log::info!("groups stuff: {:?}", self);
        let link_clone = ctx.link().clone();
        match msg {
            LeaveGroup => {}
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
            AddNewUser => {}
            AddUser(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    log::info!("add_user value: {:?}", &elem.value());
                    self.add_user = elem.value();
                }
            }
            CreateGroup => {
                link_clone.send_message(GroupsMsg::UpdateUsername);
                log::info!("making a group, username is: {:?}", &self.username);
                let username = self.username.clone();
                let group_name = self.create_group_name.clone();
                spawn_local(async move {
                    let resp = new_group_request(username, group_name).await;
                    log::info!("{:?}", &resp);
                })
            }
            UpdateUsername => {
                link_clone.send_future(async {
                    let claims = auth_flow().await.expect("umm");
                    GroupsMsg::SetUsername(claims.username)
                });
            }
            SetUsername(user) => self.username = user,
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
        { self.create_group(ctx) }
        { self.leave_group(ctx) }
        { self.add_user_to_group(ctx) }
        </div> }
    }
}
