use crate::utils::auth_flow;
use anyhow::Result;
use lazy_static::lazy_static;
use load_dotenv::load_dotenv;
use reqwasm::http::Request;
use reqwasm::http::RequestMode;
use shared_stuff::groups_stuff::BasicUsername;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {
    username: String,
}
pub enum GroupsMsg {
    CreateGroup,
    UpdateUsername,
    SetUsername(String),
}

lazy_static! {
    pub static ref ROOT_URL: &'static str = {
        load_dotenv!();
        env!("ROOT_URL")
    };
    pub static ref CREATE_GROUP_URL: String = format!("{}/create_group", *ROOT_URL);
}

pub async fn new_group_request(username: String) -> Result<()> {
    let json_body = serde_json::to_string(&BasicUsername { username })?;
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
        Self { username }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GroupsMsg::*;
        log::info!("groups stuff: {:?}", self);
        let link_clone = ctx.link().clone();
        match msg {
            CreateGroup => {
                link_clone.send_message(GroupsMsg::UpdateUsername);
                log::info!("making a group, username is: {:?}", &self.username);
                let username = self.username.clone();
                spawn_local(async move {
                    let resp = new_group_request(username).await;
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
        </div> }
    }
}
