use crate::error::Error;
use crate::utils::request_auth_flow;
use reqwasm::http::{Request, RequestMode};

use crate::LOGIN_URL;
use anyhow::Result;
use gloo_storage::{LocalStorage, Storage};
use shared_stuff::TokenResponse;
use shared_stuff::UserInfo;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_login(url: &str, body: UserInfo) -> Result<TokenResponse> {
    let userinfo = serde_json::to_string(&body)?;
    log::info!("{:?}", &userinfo);
    let resp: TokenResponse = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(userinfo)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Login {
    username: String,
    password: String,
    pub error: Option<Error>,
}
pub enum LoginMsg {
    SetUsername(InputEvent),
    SetPassword(InputEvent),
    VerifyLogin,
    SetToken(TokenResponse),
    AuthorizeCheck,
    Logout,
    Noop,
    SetError(Option<Error>),
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating login page");
        Self {
            username: String::new(),
            password: String::new(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use LoginMsg::*;
        let link_clone = ctx.link().clone();
        match msg {
            SetUsername(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.username = elem.value();
                }
            }
            SetPassword(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.password = elem.value();
                }
            }
            SetError(err) => {
                self.error = err;
            }
            Logout => {
                let storage = LocalStorage::raw();
                storage.clear().expect("problem clearing data");
                log::info!("stored some data");
            }
            Noop => {}
            AuthorizeCheck => ctx.link().send_future(async move {
                request_auth_flow().await.expect("auth flow issue");
                LoginMsg::Noop
            }),
            VerifyLogin => {
                let storage = LocalStorage::raw();
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                link_clone.send_future(async move {
                    let token = request_login(&LOGIN_URL, username.clone()).await;
                    match token {
                        Ok(tok) => {
                            storage
                                .set("username", &username.username.clone())
                                .expect("storage problem");
                            LoginMsg::SetToken(tok)
                        }
                        Err(_) => SetError(Some(Error::LogInError)),
                    }
                });
            }

            SetToken(token) => {
                let storage = LocalStorage::raw();
                storage
                    .set("access_token", &token.access_token)
                    .expect("problem setting token");
                storage
                    .set("refresh_token", &token.refresh_token.unwrap())
                    .expect("problem setting token");
                log::info!("stored some data");
            }
        }
        log::info!("{:?}", self);
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
        { self.login_html(ctx) }
        { self.authorize_html(ctx) }
        { self.logout_button(ctx) }

        </div> }
    }
}
