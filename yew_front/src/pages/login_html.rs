use crate::pages::login::{Login, LoginMsg};

use yew::prelude::*;

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
            <div class="has-background-grey-lighter">
            <div class="container column is-6 is-centered">
            <div class="card">
                <p class="is-size-4 has-text-centered pt-3 mt-3">{"Login"}</p>
                <div class="container field column is-9">
                    <p class="control has-icons-left">
                    <input class="input" type="email" placeholder="Email"
                    maxlength=50
                    oninput={ctx.link().callback(LoginMsg::SetUsername)}/>
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"email"}</i>
                    </span>
                    </p>
                </div>
                <div class="container field column is-9">
                    <p class="control has-icons-left">
                    <input class="input" type="password" placeholder="Password"
                    maxlength=50
                    oninput={ctx.link().callback(LoginMsg::SetPassword)}/>
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"lock"}</i>
                    </span>
                    </p>
                </div>
                <div class="container column is-9">
                <button
                    class="button is-primary pl-3"
                    onclick={ctx.link().callback(|_| LoginMsg::VerifyLogin)}>
                    { "Login" }
                </button>
                <a class="content is-size-6 is-pulled-right" href="url">{"forgot your password?"}</a>
                </div>
                <div class="container column is-9">
                <p class="content is-size-6">{"Don't have an account?"}
                <a class="content is-size-6" href="Register">{" Register here"}</a></p>
                </div>
            </div>
            </div>
            </div>
        }
    }
    pub fn logout_button(&self, ctx: &Context<Self>) -> Html {
        html! {            <button
            class="logout_button"
            onclick={ctx.link().callback(|_| LoginMsg::Logout)}>
            { "Logout" }
        </button> }
    }
}
