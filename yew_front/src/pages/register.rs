use crate::utils::register_request;

extern crate zxcvbn;
use zxcvbn::zxcvbn;

use crate::REGISTER_URL;
use crate::error::Error;
use shared_stuff::UserInfo;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    username: String,
    pub password: String,
    pub confirmed_password: String,
    pub password_strength: Option<u8>,
    pub error: Option<Error>,
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
            error: None,
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
                    self.confirmed_password = elem.value();
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
                if !self.repeated_password_matching() {
                    self.error = Some(Error::PasswordNotMatchError);
                } else if self.password_strength < Some(3) {
                    self.error = Some(Error::PasswordWeakError);
                } else {
                    self.error = None;
                    spawn_local(async move {
                        let resp = register_request(&REGISTER_URL, username).await;
                        match resp {
                            Ok(_a) => log::info!("success!"),
                            Err(e) => log::info!("oh no, {:?}", &e),
                        }
                    });
                }
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

impl Register {
    fn get_password_strength_estimate(&self) -> Option<u8> {
        zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score())
    }
    fn get_password_strength_text(&self) -> String {
        if self.password.is_empty() {
            return "Provide a password".to_string();
        }
        format!("Password Strength = {:?}", self.get_password_strength_estimate().unwrap())
    }
    pub fn repeated_password_matching(&self) -> bool {
        self.password.eq(&self.confirmed_password)
    }
    pub fn repeated_password_matching_text(&self) -> String {
        format!("Repeat password correct: {:?}", self.repeated_password_matching())
    }
    pub fn register_html(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <div>
                <h1> {"REGISTER"} </h1>
                <input
                    class="login_user_name"
                    placeholder="Username"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::SetUsername)}
                />
            </div>
            <div>
                <input
                    type="password"
                    class="login_user_name"
                    placeholder="Password"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::SetPassword)}
                />
                <p> {self.get_password_strength_text()} </p>
            </div>
            <div>
                <input
                    type="password"
                    class="login_user_name"
                    placeholder="Repeat password"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::ConfirmPassword)}
                />
                <p> {self.repeated_password_matching_text()} </p>
            </div>
            <div>
            <button
                class="register_password_strength"
                onclick={ctx.link().callback(|_| RegisterMsg::RegisterUser)}>
                { "Register" }
            </button>
            <p> {format!("Register error: {:?}", self.error.clone())} </p>
            </div>
        </div>
        }
    }
}


