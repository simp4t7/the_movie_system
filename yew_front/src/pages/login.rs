use crate::utils::auth_flow;
use crate::utils::{login_request, register_request};

use crate::LOGIN_URL;
use crate::REGISTER_URL;
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
}
pub enum LoginMsg {
    SetUsername(InputEvent),
    SetPassword(InputEvent),
    VerifyLogin,
    RegisterUser,
    SetToken(DoubleTokenResponse),
    AuthorizeCheck,
    Logout,
    Noop,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating login page");
        Self {
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use LoginMsg::*;
        match msg {
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
            RegisterUser => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                spawn_local(async move {
                    let resp = register_request(&REGISTER_URL, username).await;
                    match resp {
                        Ok(_a) => log::info!("success!"),
                        Err(e) => log::info!("oh no, {:?}", &e),
                    }
                });
            }
            VerifyLogin => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                let link_clone = ctx.link().clone();
                spawn_local(async move {
                    let token = login_request(&LOGIN_URL, username).await;
                    match token {
                        Ok(tok) => link_clone.send_message(LoginMsg::SetToken(tok)),
                        Err(_) => link_clone.send_message(LoginMsg::Noop),
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
        { self.register_html(ctx) }
        { self.login_html(ctx) }
        { self.authorize_html(ctx) }
        { self.logout_button(ctx) }

        </div> }
    }
}
