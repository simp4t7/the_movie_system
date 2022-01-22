use zxcvbn::zxcvbn;

use crate::error::Error;
use crate::REGISTER_URL;
use anyhow::Result;
use reqwasm::http::{Request, RequestMode};
use shared_stuff::UserInfo;
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
                //if !self.repeated_password_matching() {
                //self.error = Some(Error::PasswordNotMatchError);
                //} else if self.password_strength < Some(3) {
                //self.error = Some(Error::PasswordWeakError);
                //} else {
                //self.error = None;
                ctx.link().send_future(async move {
                    let resp = request_register(&REGISTER_URL, username).await;
                    match resp {
                        Ok(_a) => log::info!("success!"),
                        Err(e) => log::info!("oh no, {:?}", &e),
                    }
                    RegisterMsg::Noop
                });
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
