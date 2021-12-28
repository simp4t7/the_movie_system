use crate::SEARCH_URL;
use shared_stuff::ImdbQuery;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;

use crate::utils::get_search_results;
use crate::utils::image_processing;
use shared_stuff::MovieDisplay;

#[derive(Debug)]
pub enum HomeMsg {}
pub struct Home {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <p> {"new home page, who dis?"} </p>
        </div>}
    }
}
