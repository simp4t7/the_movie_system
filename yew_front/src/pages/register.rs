use crate::utils::register_request;

extern crate zxcvbn;
use zxcvbn::zxcvbn;

use crate::REGISTER_URL;
use shared_stuff::UserInfo;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    username: String,
    pub password: String,
    pub confirmed_password: String,
    pub password_strength: Option<u9>,
    error: Option<Err>,
}
pub enum RegisterMsg {
    SetUsername(InputEvent),
    SetPassword(InputEvent),
    ConfirmPassword(InputEvent),
    RegisterUser,
    Noop,
}

impl Component for Register {
    type Message = RegisterMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating login page");
        Self {
            username: String::new(),
            password: String::new(),
            confirmed_password: String::new(),
            password_strength: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use RegisterMsg::*;
        match msg {
            Noop => {}
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
            ConfirmPassword(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.password = elem.value();
                    self.password_strength = zxcvbn(&self.password, &[])
                                            .ok()
                                            .map(|estimate| estimate.score());

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
        </div> }
    }
}
