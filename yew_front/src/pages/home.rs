use crate::SEARCH_URL;
use shared_stuff::ImdbQuery;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::TargetCast;

use crate::utils::get_search_results;
use shared_stuff::MovieDisplay;

#[derive(Debug)]
pub enum HomeMsg {
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    Error(String),
}
pub struct Home {
    pub movies: Vec<MovieDisplay>,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        Self { movies: vec![] }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use HomeMsg::QueryAutocomplete;
        let link_clone = ctx.link().clone();
        match msg {
            QueryAutocomplete(text) => {
                // Shouldn't do it if the text is empty, but handle this better probably...
                if text.current_target().is_some() {
                    spawn_local(async move {
                        if let Some(elem) = text.target_dyn_into::<HtmlInputElement>() {
                            let query = ImdbQuery {
                                query: elem.value(),
                            };

                            match get_search_results(&SEARCH_URL, query).await {
                                Ok(resp) => {
                                    link_clone.send_message(HomeMsg::UpdateAutocomplete(resp))
                                }
                                Err(err_msg) => {
                                    link_clone.send_message(HomeMsg::Error(err_msg.to_string()))
                                }
                            }
                        }
                    });
                }
            }

            HomeMsg::UpdateAutocomplete(movies) => {
                log::info!("{:?}", &movies);
                self.movies = movies;
            }
            HomeMsg::Error(err_msg) => {
                log::info!("{:?}", &err_msg);
            }
        };
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            {self.search_bar(ctx)}
        </div>}
    }
}
