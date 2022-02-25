use zxcvbn::zxcvbn;

use crate::error::Error;
use crate::REGISTER_URL;
use anyhow::Result;
use reqwasm::http::{Request, RequestMode};
use shared_stuff::auth_structs::UserInfo;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub async fn request_register(url: &str, body: UserInfo) -> Result<()> {
    let userinfo = serde_json::to_string(&body)?;
    log::info!("{:?}", &userinfo);
    let resp = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(userinfo)
        .send()
        .await?;
    log::info!("{:?}", &resp);
    Ok(())
}

impl Register {
    pub fn full_validation_check(&mut self) {
        self.validate_password_strength();
        self.validate_email();
        self.validate_repeated_pass();
        if self.validation.password_strong
            && self.validation.email_valid
            && self.validation.password_match
        {
            self.validation.all_valid = true;
        }
    }
    pub fn validate_password_strength(&mut self) {
        log::info!("checking password strength");
        let score = zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score());
        self.validation.password_score = score;
        match score {
            Some(4) => self.validation.password_strong = true,
            Some(0) | Some(1) | Some(2) | Some(3) => self.validation.password_strong = false,
            _ => self.validation.password_strong = false,
        }
    }
    pub fn validate_email(&mut self) {
        match validator::validate_email(&self.username) {
            true => self.validation.email_valid = true,
            false => self.validation.email_valid = false,
        }
    }
    pub fn validate_repeated_pass(&mut self) {
        match self.password.eq(&self.confirmed_password) {
            true => self.validation.password_match = true,
            false => self.validation.password_match = false,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct RegisterValidation {
    pub all_valid: bool,
    pub email_valid: bool,
    pub password_strong: bool,
    pub password_score: Option<u8>,
    pub password_match: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    pub username: String,
    pub validation: RegisterValidation,
    pub password: String,
    pub confirmed_password: String,
    pub password_strength: Option<u8>,
    pub error: Option<Error>,
}

#[derive(Debug, PartialEq, Clone)]
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
            validation: RegisterValidation::default(),
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
                self.validate_email();
            }
            SetPassword(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.password = elem.value();
                }
                self.validate_password_strength();
            }
            ConfirmPassword(text) => {
                if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                    self.confirmed_password = elem.value();
                }
                self.validate_repeated_pass();
            }
            RegisterUser => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                self.full_validation_check();
                if self.validation.all_valid {
                    ctx.link().send_future(async move {
                        let resp = request_register(&REGISTER_URL, username).await;
                        match resp {
                            Ok(_a) => log::info!("success!"),
                            Err(e) => log::info!("oh no, {:?}", &e),
                        }
                        RegisterMsg::Noop
                    });
                }
                //}
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
