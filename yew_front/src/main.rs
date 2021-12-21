use crate::utils::auth_flow;

use lazy_static::lazy_static;
use load_dotenv::load_dotenv;

use wasm_bindgen_futures::spawn_local;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use yew::{html, Component, Context, Html};

pub mod html_gen;
pub mod pages;
pub mod utils;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Home,
    #[at("register")]
    Register,
    #[at("/404")]
    NotFound,
}
#[function_component(Main)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AuthStatus {
    pub status: bool,
    pub username: Option<String>,
    pub exp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Properties)]
pub struct App {
    username: Option<String>,
}

pub enum AppMsg {
    AuthCallback,
    UpdateAuth(AuthStatus),
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AppMsg::AuthCallback);
        App { username: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link_clone = ctx.link().clone();
        match msg {
            AppMsg::AuthCallback => spawn_local(async move {
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
                link_clone.send_message(AppMsg::UpdateAuth(login_status));
            }),
            AppMsg::UpdateAuth(login) => {
                self.username = login.username;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::info!("{:?}", &self);
        html! {
            <BrowserRouter>
            <main>
            <NavBar username ={self.username.clone()}/>
            <Switch<Route> render={Switch::render(switch)} />
            </main>
            </BrowserRouter>

        }
    }
}

#[function_component(NavBar)]
pub fn nav_bar(app: &App) -> Html {
    match app.username.clone() {
        Some(user) => {
            html! {
                <div class="nav_bar">
                    <ul class="nav_bar">
                        <li><a href="/">{"Home"}</a></li>
                        <li><a href="/contact">{"Contact"}</a></li>
                        <li style="float:right"><a href="/about">{"About"}</a></li>
                        <li style="float:right"><a href="/login">{user}</a></li>
                    </ul>
                </div>
            }
        }
        None => {
            html! {
                <div class="nav_bar">
                    <ul class="nav_bar">
                        <li><a href="/">{"Home"}</a></li>
                        <li><a href="/contact">{"Contact"}</a></li>
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
        Route::Register => html!{<Register />},
        //TODO! something for bad urls?
        Route::NotFound => html!{},
    }}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
