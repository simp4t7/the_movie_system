use crate::pages::register::{Register, RegisterMsg};

use yew::prelude::*;
use zxcvbn::zxcvbn;

impl Register {
    //fn get_password_strength_estimate(&self) -> Option<u8> {
    //zxcvbn(&self.password, &[])
    //.ok()
    //.map(|estimate| estimate.score())
    //}
    //fn get_password_strength_text(&self) -> String {
    //if self.password.is_empty() {
    //return "Provide a password".to_string();
    //}
    //format!(
    //"Password Strength = {:?}",
    //self.get_password_strength_estimate().unwrap()
    //)
    //}
    //pub fn repeated_password_matching(&self) -> bool {
    //self.password.eq(&self.confirmed_password)
    //}
    //pub fn repeated_password_matching_text(&self) -> String {
    //format!(
    //"Repeat password correct: {:?}",
    //self.repeated_password_matching()
    //)
    //}
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

                  { match self.validation.password_strong {
                        true => html!{
                            <input class="input is-primary" type="password" placeholder="Password"
                            maxlength=50
                            oninput={ctx.link().callback(RegisterMsg::SetPassword)}/>
                        },
                        false => html!{<input class="input" type="password" placeholder="Password"
                            maxlength=50
                            oninput={ctx.link().callback(RegisterMsg::SetPassword)}/>
                        },
                        }
                  }
                    <span class="icon is-small is-left">
                    <i class="material-icons">{"lock"}</i>
                    </span>
                    </p>
                </div>
                <div class="container field column is-9">
                    {
                    match self.validation.password_score {
                        Some(0) => html!{<progress class="progress is-danger" value="0" max="100"></progress>},
                        Some(1) => html!{<progress class="progress is-danger" value="25" max="100"></progress>},
                        Some(2) => html!{<progress class="progress is-warning" value="50" max="100"></progress>},
                        Some(3) => html!{<progress class="progress is-warning" value="75" max="100"></progress>},
                        Some(4) => html!{<progress class="progress is-primary" value="100" max="100"></progress>},
                        _ => html!{<progress class="progress is-primary" value="0" max="100"></progress>},

                    }
                }
                </div>
                <div class="container field column is-9">
                  <p class="control has-icons-left">
                  { match self.validation.password_match {
                        true => html!{
                            <input class="input is-primary" type="password" placeholder="Repeat Password"
                            maxlength=50
                            oninput={ctx.link().callback(RegisterMsg::ConfirmPassword)}/>
                        },
                        false => html!{<input class="input" type="password" placeholder="Repeat Password"
                            maxlength=50
                            oninput={ctx.link().callback(RegisterMsg::ConfirmPassword)}/>
                        },
                        }
                  }
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
                <p> {format!("email valid: {:?}", self.validation.email_valid)} </p>
                <p> {format!("password_strong: {:?}", self.validation.password_strong)} </p>
                <p> {format!("password strength: {:?}", self.validation.password_score)} </p>
                <p> {format!("correctly repeated password: {:?}", self.validation.password_match)} </p>
            </div>
            </div>
            </div>
        }
    }
}
