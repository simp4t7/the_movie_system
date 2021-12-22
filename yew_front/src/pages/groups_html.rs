use crate::pages::groups::Groups;
use crate::pages::groups::GroupsMsg;
use yew::prelude::*;

impl Groups {
    pub fn create_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Authorize"} </h1>
            <button
                class="authorize_button"
                onclick={ctx.link().callback(|_| GroupsMsg::CreateGroup)}>
                { "Authorize" }
            </button>
        </div>
        }
    }
}
