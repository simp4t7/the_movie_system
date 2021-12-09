use crate::jwt::token_stuff;
use lazy_static::lazy_static;
use load_dotenv::load_dotenv;
use yew::prelude::*;
use yew_router::prelude::*;

pub mod html_gen;
pub mod jwt;
pub mod pages;
pub mod utils;

use pages::home::Home;
use pages::login::Login;

lazy_static! {
    pub static ref SEARCH_URL: &'static str = {
        load_dotenv!();
        env!("SEARCH_URL")
    };
    pub static ref LOGIN_URL: &'static str = {
        load_dotenv!();
        env!("LOGIN_URL")
    };
    pub static ref REGISTER_URL: &'static str = {
        load_dotenv!();
        env!("REGISTER_URL")
    };
}

#[derive(Switch, Clone)]
pub enum Route {
    #[to = "/login"]
    Login,
    #[to = "/"]
    Home,
}
struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
            <div>
            {nav_bar()}
            {router_function()}
            </div>

        }
    }
}

pub fn nav_bar() -> Html {
    html! {

        <div class="nav_bar">
            <ul class="nav_bar">
                <li><a href="/">{"Home"}</a></li>
                <li><a href="/login">{"Login"}</a></li>
                <li><a href="/contact">{"Contact"}</a></li>
                <li style="float:right"><a href="/about">{"About"}</a></li>
            </ul>
        </div>

    }
}

pub fn router_function() -> Html {
    html! {                <Router<Route>
    render = Router::render(|switch: Route| {
        match switch {
            Route::Home => html!{<Home/>},
            Route::Login => html!{<Login/>},
        }})/>
    }
}

fn main() {
    //let x = token_stuff();
    //log::info!("{:?}", &x);
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
