use std::collections::HashSet;
use yew::prelude::*;

pub enum TheSystemMsg {}

pub struct TheSystem {
    pub ready_status: bool,
    pub users_not_ready: HashSet<String>,
    pub users_ready: HashSet<String>,
}

impl Component for TheSystem {
    type Message = TheSystemMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let ready_status = false;
        let users_not_ready = HashSet::new();
        let users_ready = HashSet::new();
        Self {
            ready_status,
            users_not_ready,
            users_ready,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            {self.system_ready_status(ctx)}
        }
    }
}
