



use yew::prelude::*;






#[derive(Debug)]
pub enum HomeMsg {}
pub struct Home {}

impl Component for Home {
    type Message = HomeMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <p> {"new home page, who dis?"} </p>
        </div>}
    }
}
