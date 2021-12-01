use crate::utils::get_search_results;
use crate::utils::image_processing;
use crate::InputData;
use wasm_bindgen_futures::spawn_local;
use yew::html;
use yew::html::Html;

use crate::pages::home::Home;
use crate::pages::home::HomeMsg;
use crate::pages::login::Login;
use crate::pages::login::LoginMsg;

impl Login {
    pub fn login_html(&self) -> Html {
        html! {
        <div>
            <h1> {"LOGIN"} </h1>
            <input
                class="login_user_name"
                placeholder="Username"
                maxlength=50
                oninput={self.link.callback(
                    |text: InputData|
                    LoginMsg::SetUsername(text))}
            />
            <input
                type="password"
                class="login_user_name"
                placeholder="Password"
                maxlength=50
                oninput={self.link.callback(
                    |text: InputData|
                    LoginMsg::SetPassword(text))}
            />
            <button
                class="login_user_name"
                onclick=self.link.callback(|_| LoginMsg::VerifyLogin)>
                { "Login" }
            </button>
        </div>}
    }
    pub fn register_html(&self) -> Html {
        html! {
        <div>
            <h1> {"REGISTER"} </h1>
            <input
                class="login_user_name"
                placeholder="Username"
                maxlength=50
                oninput={self.link.callback(
                    |text: InputData|
                    LoginMsg::SetUsername(text))}
            />
            <input
                type="password"
                class="login_user_name"
                placeholder="Password"
                maxlength=50
                oninput={self.link.callback(
                    |text: InputData|
                    LoginMsg::SetPassword(text))}
            />
            <button
                class="login_user_name"
                onclick=self.link.callback(|_| LoginMsg::RegisterUser)>
                { "Login" }
            </button>
        </div>}
    }
}

impl Home {
    pub fn search_results(&self) -> Html {
        {
            self.movies
                .iter()
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
    pub fn search_bar(&self) -> Html {
        html! {
                    <div class="movie_search_div">
                        <input
                        class="movie_search"
                        placeholder="movie search"
                        maxlength=50
                        oninput={self.link.callback(|text: InputData|
                            HomeMsg::QueryAutocomplete(text))}
                        />
                    <div class="search_results">
                    <ul>
                    {self.search_results()}
                    </ul>
                    </div>
                    </div>
        }
    }
}
