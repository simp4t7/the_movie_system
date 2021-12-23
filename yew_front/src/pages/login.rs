use crate::utils::auth_flow;
use crate::utils::login_request;
use crate::error::Error;

use crate::LOGIN_URL;
use gloo_storage::LocalStorage;
use gloo_storage::Storage;
use shared_stuff::DoubleTokenResponse;
use shared_stuff::UserInfo;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

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
    SetToken(DoubleTokenResponse),
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
            SetError(errorOption) => {
                self.error = errorOption;
            }
            Logout => {
                let storage = LocalStorage::raw();
                storage
                    .delete("access_token")
                    .expect("problem deleting access token");
                storage
                    .delete("refresh_token")
                    .expect("problem deleting refresh token");
                log::info!("stored some data");
            }
            Noop => {}
            AuthorizeCheck => spawn_local(async move {
                let _x = auth_flow().await;
            }),
            VerifyLogin => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                let link_clone = ctx.link().clone();
                let link_clone_for_error = ctx.link().clone();
                spawn_local(async move {
                    let token = login_request(&LOGIN_URL, username).await;
                    match token {
                        Ok(tok) => {
                            link_clone.send_message(LoginMsg::SetToken(tok));
                            link_clone_for_error.send_message(SetError(None));
                        }
                        Err(_) => {
                            link_clone.send_message(LoginMsg::Noop); 
                            link_clone_for_error.send_message(SetError(Some(Error::LogInError)));
                        }
                    }
                });
            }

            SetToken(token) => {
                let storage = LocalStorage::raw();
                storage
                    .set("access_token", &token.access_token)
                    .expect("problem setting token");
                storage
                    .set("refresh_token", &token.refresh_token)
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

impl Login {
    pub fn authorize_html(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Authorize"} </h1>
            <button
                class="authorize_button"
                onclick={ctx.link().callback(|_| LoginMsg::AuthorizeCheck)}>
                { "Authorize" }
            </button>
        </div>
        }
    }
    pub fn login_html(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"LOGIN"} </h1>
            <input
                class="login_user_name"
                placeholder="Username"
                maxlength=50
                oninput={ctx.link().callback(LoginMsg::SetUsername)}
            />
            <input
                type="password"
                class="login_user_name"
                placeholder="Password"
                maxlength=50
                oninput={ctx.link().callback(LoginMsg::SetPassword)}
            />
            <button
                class="login_user_name"
                onclick={ctx.link().callback(|_| LoginMsg::VerifyLogin)}>
                { "Login" }
            </button>
            <p> {format!("Login error: {:?}", self.error.clone())} </p>
        </div>}
    }
    pub fn logout_button(&self, ctx: &Context<Self>) -> Html {
        html! {            <button
            class="logout_button"
            onclick={ctx.link().callback(|_| LoginMsg::Logout)}>
            { "Logout" }
        </button> }
    }
}
