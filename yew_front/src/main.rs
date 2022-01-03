use crate::utils::auth_flow;

use lazy_static::lazy_static;
use load_dotenv::load_dotenv;

use gloo_storage::LocalStorage;
use gloo_storage::Storage;

use yew::functional::*;

use yew_router::prelude::*;

use yew::{html, Component, Context, Html};

pub mod error;
pub mod pages;
pub mod utils;

use pages::add_movies::AddMovies;
use pages::groups::Groups;
use pages::home::Home;
use pages::login::Login;
use pages::register::Register;

lazy_static! {
    pub static ref ROOT_URL: &'static str = {
        load_dotenv!();
        env!("ROOT_URL")
    };
    pub static ref SEARCH_URL: String = format!("{}/search", *ROOT_URL);
    pub static ref LOGIN_URL: String = format!("{}/login", *ROOT_URL);
    pub static ref REGISTER_URL: String = format!("{}/register", *ROOT_URL);
    pub static ref ACCESS_URL: String = format!("{}/access_auth", *ROOT_URL);
    pub static ref REFRESH_URL: String = format!("{}/refresh_auth", *ROOT_URL);
    pub static ref GET_GROUP_MOVIES_URL: String = format!("{}/get_group_movies", *ROOT_URL);
    pub static ref SAVE_GROUP_MOVIES_URL: String = format!("{}/save_group_movies", *ROOT_URL);
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("register")]
    Register,
    #[at("add_movies")]
    AddMovies,
    #[at("groups")]
    Groups,
    #[at("/404")]
    NotFound,
}
#[derive(Debug, PartialEq, Clone)]
pub struct GlobalState {
    pub username: String,
    pub groups: Vec<String>,
    pub initialized: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AuthStatus {
    pub status: bool,
    pub username: Option<String>,
    pub exp: Option<i64>,
}

#[derive(Clone, PartialEq)]
pub struct App {}

pub enum AppMsg {
    AuthCallback,
    UpdateAuth(AuthStatus),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AppMsg::AuthCallback);

        App {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::AuthCallback => ctx.link().send_future(async move {
                log::info!("inside auth callback");
                let request_claims = auth_flow().await;
                let login_status = match request_claims {
                    Ok(claims) => AuthStatus {
                        status: true,
                        username: Some(claims.username),
                        exp: Some(claims.exp),
                    },
                    Err(_) => AuthStatus {
                        status: false,
                        username: None,
                        exp: None,
                    },
                };
                log::info!("auth callback, {:?}", &login_status);
                AppMsg::UpdateAuth(login_status)
            }),
            AppMsg::UpdateAuth(login) => {
                log::info!("inside update auth");
                if let Some(username) = login.username {
                    let storage = LocalStorage::raw();
                    storage
                        .set("username", &username)
                        .expect("problem with local storage");
                }
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
            <main>
            <NavBar/>
            <Switch<Route> render={Switch::render(switch)} />
            </main>
            </BrowserRouter>

        }
    }
}

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let storage = LocalStorage::raw();
    match storage.get("username") {
        Ok(user) if user.is_some() => {
            html! {
            <div class="nav_bar">
            <ul class="nav_bar">
            <li><a href="/">{"Home"}</a></li>
            <li><a href="/groups">{"Group"}</a></li>
            <li><a href="/add_movies">{"Add Movies"}</a></li>
            <li><a href="/register">{"Register"}</a></li>
            <li style="float:right"><a href="/about">{"About"}</a></li>
            <li style="float:right"><a href="/login">{user.unwrap()}</a></li>
            </ul>
            </div>
            }
        }
        _ => {
            html! {
            <div class="nav_bar">
            <ul class="nav_bar">
            <li><a href="/">{"Home"}</a></li>
            <li><a href="/groups">{"Groups"}</a></li>
            <li><a href="/add_movies">{"Add Movies"}</a></li>
            <li><a href="/register">{"Register"}</a></li>
            <li style="float:right"><a href="/about">{"About"}</a></li>
            <li style="float:right"><a href="/login">{"Login"}</a></li>
            </ul>
            </div>
            }
        }
    }
}

fn switch(routes: &Route) -> Html {
    html! {
    match routes {
        Route::Home => html!{<Home />},
        Route::Login => html!{<Login />},
        Route::Groups => html!{<Groups />},
        Route::Register => html!{<Register />},
        Route::AddMovies => html!{<AddMovies />},
        //TODO! something for bad urls?
        Route::NotFound => html!{},
    }}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
