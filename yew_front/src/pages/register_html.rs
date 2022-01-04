use crate::pages::register::{Register, RegisterMsg};
use anyhow::Result;
use reqwasm::http::{Request, RequestMode};
use shared_stuff::UserInfo;
use yew::prelude::*;
use zxcvbn::zxcvbn;

pub async fn register_request(url: &str, body: UserInfo) -> Result<()> {
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
    fn get_password_strength_estimate(&self) -> Option<u8> {
        zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score())
    }
    fn get_password_strength_text(&self) -> String {
        if self.password.is_empty() {
            return "Provide a password".to_string();
        }
        format!(
            "Password Strength = {:?}",
            self.get_password_strength_estimate().unwrap()
        )
    }
    pub fn repeated_password_matching(&self) -> bool {
        self.password.eq(&self.confirmed_password)
    }
    pub fn repeated_password_matching_text(&self) -> String {
        format!(
            "Repeat password correct: {:?}",
            self.repeated_password_matching()
        )
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
