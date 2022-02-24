use crate::pages::register::{Register, RegisterMsg};

use yew::prelude::*;
use zxcvbn::zxcvbn;

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
            <div class="has-background-grey-lighter">
            <div class="container column is-6 is-centered">
            <div class="card">
                <p class="is-size-4 has-text-centered pt-3 mt-3">{"Register"}</p>
                <div class="container field column is-9">
                    <p class="control has-icons-left">
                    <input class="input" type="email" placeholder="Email"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::SetUsername)}/>
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"email"}</i>
                    </span>
                    </p>
                </div>
                <div class="container field column is-9">
                  <p class="control has-icons-left">
                    <input class="input" type="password" placeholder="Password"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::SetPassword)}/>
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"lock"}</i>
                    </span>
                    </p>
                </div>
                <div class="container field column is-9">
                  <p class="control has-icons-left">
                    <input class="input" type="password" placeholder="Repeat Password"
                    maxlength=50
                    oninput={ctx.link().callback(RegisterMsg::ConfirmPassword)}/>
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"lock"}</i>
                    </span>
                </p>
                </div>
                <div class="container field column is-9">
                <button
                    class="button is-primary"
                    onclick={ctx.link().callback(|_| RegisterMsg::RegisterUser)}>
                    { "Register" }
                </button>
                </div>
                <div class="container column is-6 is-centered">
                </div>

                <p> {self.get_password_strength_text()} </p>
                <p> {self.repeated_password_matching_text()} </p>
                <p> {format!("Register error: {:?}", self.error.clone())} </p>
            </div>
            </div>
            </div>
        }
    }
}
