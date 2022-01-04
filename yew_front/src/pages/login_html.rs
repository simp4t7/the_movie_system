use crate::pages::login::{Login, LoginMsg};
use anyhow::Result;
use reqwasm::http::{Request, RequestMode};
use shared_stuff::{DoubleTokenResponse, UserInfo};
use yew::prelude::*;

pub async fn login_request(url: &str, body: UserInfo) -> Result<DoubleTokenResponse> {
    let userinfo = serde_json::to_string(&body)?;
    log::info!("{:?}", &userinfo);
    let resp: DoubleTokenResponse = Request::post(url)
        .header("content-type", "application/json; charset=UTF-8")
        .mode(RequestMode::Cors)
        .body(userinfo)
        .send()
        .await?
        .json()
        .await?;

    Ok(resp)
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
            <p> {"Don't have an account? "} <a href="/register">{"Register here"}</a></p>
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
