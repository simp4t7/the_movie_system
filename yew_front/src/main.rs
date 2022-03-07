use crate::auth_requests::request_auth_flow;

use lazy_static::lazy_static;
use load_dotenv::load_dotenv;

use gloo_storage::LocalStorage;
use gloo_storage::Storage;

use yew::functional::*;

use yew_router::prelude::*;

use yew::{html, Component, Context, Html, Properties};

pub mod auth_requests;
pub mod error;
pub mod pages;
pub mod shared_requests;
pub mod utils;

use pages::all_groups::AllGroups;
use pages::group::Group;
use pages::home::Home;
use pages::login::Login;
use pages::register::Register;
use pages::system::System;
use pages::user::User;

lazy_static! {
    pub static ref ROOT_URL: &'static str = {
        load_dotenv!();
        env!("ROOT_URL")
    };
    pub static ref CORS_ORIGIN: &'static str = {
        load_dotenv!();
        env!("CORS_ORIGIN")
    };
    pub static ref SEARCH_URL: String = format!("{}/search", *ROOT_URL);
    pub static ref LOGIN_URL: String = format!("{}/login", *ROOT_URL);
    pub static ref REGISTER_URL: String = format!("{}/register", *ROOT_URL);
    pub static ref ACCESS_URL: String = format!("{}/access_auth", *ROOT_URL);
    pub static ref REFRESH_URL: String = format!("{}/refresh_auth", *ROOT_URL);
    pub static ref UPDATE_GROUP_DATA_URL: String = format!("{}/update_group_data", *ROOT_URL);
    //pub static ref GET_GROUP_MOVIES_URL: String = format!("{}/get_group_movies", *ROOT_URL);
    //pub static ref SAVE_GROUP_MOVIES_URL: String = format!("{}/save_group_movies", *ROOT_URL);
    pub static ref CREATE_GROUP_URL: String = format!("{}/create_group", *ROOT_URL);
    pub static ref LEAVE_GROUP_URL: String = format!("{}/leave_group", *ROOT_URL);
    pub static ref ADD_USER_URL: String = format!("{}/add_user", *ROOT_URL);
    pub static ref GET_ALL_GROUPS_URL: String = format!("{}/get_all_groups", *ROOT_URL);
    pub static ref GET_GROUP_DATA_URL: String = format!("{}/get_group_data", *ROOT_URL);
    pub static ref GET_USER_PROFILE: String = format!("{}/get_user_profile", *ROOT_URL);
}

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("register")]
    Register,
    #[at("groups")]
    AllGroups,
    #[at("/user/:username")]
    User { username: String },
    #[at("/group/:group_id")]
    Group { group_id: String },
    #[at("/system/:group_id")]
    System { group_id: String },
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

#[derive(PartialEq, Clone)]
pub enum AppMsg {
    AuthCallback,
    UpdateAuth(AuthStatus),
    Logout,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        //        ctx.link().send_message(AppMsg::AuthCallback);

        App {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::Logout => {
                let storage = LocalStorage::raw();
                storage.clear().expect("problem clearing data");
                log::info!("stored some data");
            }
            AppMsg::AuthCallback => ctx.link().send_future(async move {
                log::info!("inside auth callback");
                let request_claims = request_auth_flow().await;
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
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
            <main>
            {self.new_nav_bar(ctx)}
            <Switch<Route> render={Switch::render(switch)} />
            </main>
            </BrowserRouter>

        }
    }
}

impl App {
    pub fn new_nav_bar(&self, ctx: &Context<Self>) -> Html {
        html! {
            <nav class="navbar is-primary is-fixed-top" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <a role="button"
                       class="navbar-burger"
                       aria-label="menu"
                       aria-expanded="false"
                       data-target="navbar-main">
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    <span aria-hidden="true"></span>
                    </a>
                </div>
                <div id="navbar-main" class="navbar-menu">
                <div class="navbar-start">
                    <a class="navbar-item" href="/">{"Home"}</a>
                    <a class="navbar-item" href="/groups">{"Groups"}</a>
                <div class="navbar-item has-dropdown is-hoverable">
                    <a class="navbar-link">{"More"}</a>
                <div class="navbar-dropdown">
                    <a class="navbar-item">{"Stuff"}</a>
                    <a class="navbar-item">{"More Stuff"}</a>
                    <a class="navbar-item">{"Yeah, more"}</a>
                    <hr class="navbar-divider"/>
                    <a class="navbar-item">{"Report an issue"}</a>
                </div>
                </div>
                </div>
                <div class="navbar-end">
                <div class="navbar-item">
                {
                match LocalStorage::raw().get("username").expect("storage issue") {
                    Some(user) => html!{
                    <div class="buttons">
                        <a class="button is-primary" href={format!("/user/{}", user)}><strong>{user}</strong></a>
                        <a class="button is-light"
                    onclick={ctx.link().callback(|_| AppMsg::Logout)}
                        >{"Logout"}</a>
                    </div>
                    },
                    None => html! {
                    <div class="buttons">
                        <a class="button is-primary" href="/register"><strong>{"Register"}</strong></a>
                        <a class="button is-light" href="/login">{"Login"}</a>
                    </div>
                    }
                } }
                </div>
                </div>
                </div>
            </nav>
        }
    }
}
fn switch(routes: &Route) -> Html {
    html! {
    match routes {
        Route::Home => html!{<Home />},
        Route::Login => html!{<Login />},
        Route::AllGroups => html!{<AllGroups />},
        Route::Register => html!{<Register />},
        Route::User { username }=> html!{<User username={username.clone()} />},
        Route::Group { group_id } => html!{<Group id={group_id.clone()}/>},
        Route::System { group_id } => html!{<System id={group_id.clone()}/>},
        //TODO! something for bad urls?
        Route::NotFound => html!{},
    }}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
