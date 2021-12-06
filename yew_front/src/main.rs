use yew::prelude::*;
use yew_router::prelude::*;

pub mod html_gen;
pub mod pages;
pub mod utils;

use pages::home::Home;
use pages::login::Login;

pub const SEARCH_URL: &str = "http://192.168.137.107:3030/search";
pub const LOGIN_URL: &str = "http://192.168.137.107:3030/login";
pub const REGISTER_URL: &str = "http://192.168.107.13:3030/register";
//pub const SEARCH_URL: &str = "http://0.0.0.0:3030/search";
//pub const LOGIN_URL: &str = "http://0.0.0.0:3030/login";
//pub const REGISTER_URL: &str = "http://0.0.0.0:3030/register";

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
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
