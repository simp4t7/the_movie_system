use shared_stuff::{ImdbQuery, MovieDisplay, YewMovieDisplay};
use std::collections::{HashMap, HashSet};
use yew::prelude::*;

#[derive(Properties, Debug, PartialEq, Clone)]
pub struct Props {
    pub id: String,
}

pub enum GroupMsg {
    Noop,
    Default,
    SetGroupName,
    GetMovies,
    Error(String),
}

pub struct Group {
    /*
    pub autocomplete_movies: HashMap<String, MovieDisplay>,
    pub group_name: String,
    pub group_members: String,
    pub added_movies: HashMap<String, YewMovieDisplay>,
*/}

impl Component for Group {
    type Message = GroupMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        log::info!("creating group page for group_id: {}", &ctx.props().id);
        ctx.link().send_message(GroupMsg::GetMovies);

        Self {
            /*
            autocomplete_movies: HashMap::new(),
            group_name: String::from(""),
            added_movies: HashMap::new(),
            */
        }
    }

    /*
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GroupMsg::*;
        let link_clone = ctx.link().clone();
        let group_id = &ctx.props().id.clone();

        match msg {
            GetMovies => {
                let movies = self.added_movies.clone();
                link_clone.send_future(async move {
                    let resp = request_group_movie_list(group_id).await;
                    log::info!("resp is: {:?}", &resp);
                    GetMoviesMsg::Noop
            }

        }

        true
    }
    */

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }
}
