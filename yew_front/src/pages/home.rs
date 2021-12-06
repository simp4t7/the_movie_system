use crate::SEARCH_URL;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::utils::get_search_results;
use shared_stuff::MovieDisplay;

//mod html_gen;
//mod utils;

#[derive(Debug)]
pub enum HomeMsg {
    QueryAutocomplete(InputData),
    UpdateAutocomplete(Vec<MovieDisplay>),
    Error(String),
}
pub struct Home {
    pub link: ComponentLink<Self>,
    pub movies: Vec<MovieDisplay>,
}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            movies: vec![],
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use HomeMsg::QueryAutocomplete;
        let link_clone = self.link.clone();
        match msg {
            QueryAutocomplete(text) => {
                // Shouldn't do it if the text is empty, but handle this better probably...
                if !text.value.is_empty() {
                    spawn_local(async move {
                        match get_search_results(SEARCH_URL, text.value.into()).await {
                            Ok(resp) => link_clone.send_message(HomeMsg::UpdateAutocomplete(resp)),
                            Err(err_msg) => {
                                link_clone.send_message(HomeMsg::Error(err_msg.to_string()))
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
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {
        <div>
            {self.search_bar()}
        </div>}
    }
}
