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
pub enum AddMoviesMsg {
    QueryAutocomplete(InputEvent),
    UpdateAutocomplete(Vec<MovieDisplay>),
    AddedMovies(Vec<MovieDisplay>),
    Error(String),
}
pub struct AddMovies {
    pub autocomplete_movies: Vec<MovieDisplay>,
    pub added_movies: Vec<MovieDisplay>,
}

impl Component for AddMovies {
    type Message = AddMoviesMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating search page");
        Self {
            autocomplete_movies: vec![],
            added_movies: vec![],
        }
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use AddMoviesMsg::*;
        let link_clone = ctx.link().clone();
        match msg {
            AddedMovies(movies) => {}
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
                                    link_clone.send_message(AddMoviesMsg::UpdateAutocomplete(resp))
                                }
                                Err(err_msg) => link_clone
                                    .send_message(AddMoviesMsg::Error(err_msg.to_string())),
                            }
                        }
                    });
                }
            }

            AddMoviesMsg::UpdateAutocomplete(movies) => {
                log::info!("{:?}", &movies);
                self.autocomplete_movies = movies;
            }
            AddMoviesMsg::Error(err_msg) => {
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
            {self.add_stuff(ctx)}
        </div>}
    }
}
