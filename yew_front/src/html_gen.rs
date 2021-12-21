extern crate zxcvbn;
use zxcvbn::zxcvbn;
use crate::utils::image_processing;

use yew::html;
use yew::html::Html;
use yew::Context;

use crate::pages::home::Home;
use crate::pages::home::HomeMsg;
use crate::pages::login::Login;
use crate::pages::login::LoginMsg;

impl Login {
    fn get_password_strength_estimate(&self) -> Option<u8> {
        zxcvbn(&self.password, &[])
            .ok()
            .map(|estimate| estimate.score())
    }
    fn get_password_strength_text(&self) -> String {
        if self.password.is_empty() {
            return "Provide a password".to_string();
        }
        log::info!("password is: {:?}", &self.password);
        let estimate_text = match self.get_password_strength_estimate().unwrap_or(0) {
            0 => "That's a password?",
            1 => "You can do a lot better.",
            2 => "Meh",
            3 => "Good",
            _ => "Great!",
        };
        format!("Complexity = {}", estimate_text)
    }
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
        </div>}
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
                    oninput={ctx.link().callback(LoginMsg::SetUsername)}
                />
            </div>
            <div>
                <input
                    type="password"
                    class="login_user_name"
                    placeholder="Password"
                    maxlength=50
                    oninput={ctx.link().callback(LoginMsg::SetPassword)}
                />
                <p> {self.get_password_strength_text()} </p>
            </div>
            <div>
                <input
                    type="password"
                    class="login_user_name"
                    placeholder="Repeat password"
                    maxlength=50
                    oninput={ctx.link().callback(LoginMsg::ConfirmPassword)}
                />
            </div>
            <div>
            <button
                class="register_password_strength"
                onclick={ctx.link().callback(|_| LoginMsg::RegisterUser)}>
                { "Login" }
            </button>
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

impl Home {
    pub fn search_results(&self, _ctx: &Context<Self>) -> Html {
        {
            self.movies
                .iter()
                // Still not handling the no images nicely?
                .map(|movie| {
                    if movie.movie_images.is_some() {
                        html! {
                        <li class="search_results_li">
                            {&movie.movie_title}
                            {&movie.movie_year.unwrap()}
                            <img src={image_processing(movie.movie_images.as_ref())}/>
                        </li>}
                    } else {
                        html! {
                        <li>
                            {&movie.movie_title}
                            {&movie.movie_year.unwrap()}
                        </li>}
                    }
                })
                .collect::<Html>()
        }
    }
    pub fn search_bar(&self, ctx: &Context<Self>) -> Html {
        html! {
                    <div class="movie_search_div">
                        <input
                        class="movie_search"
                        placeholder="movie search"
                        maxlength=50
                        oninput={ctx.link().callback(HomeMsg::QueryAutocomplete)}
                        />
                    <div class="search_results">
                    <ul>
                    {self.search_results(ctx)}
                    </ul>
                    </div>
                    </div>
        }
    }
}
