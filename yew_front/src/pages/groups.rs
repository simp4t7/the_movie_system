use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Groups {}
pub enum GroupsMsg {
    CreateGroup,
}

impl Component for Groups {
    type Message = GroupsMsg;
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("creating groups page");
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        use GroupsMsg::*;
        log::info!("{:?}", self);
        match msg {
            CreateGroup => {}
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
        { self.create_group(ctx) }
        </div> }
    }
}
