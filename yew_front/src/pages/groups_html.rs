use crate::pages::groups::Groups;
use crate::pages::groups::GroupsMsg;
use yew::prelude::*;

impl Groups {
    pub fn create_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Create Group"} </h1>
            <input
                class="add_group"
                placeholder="group name"
                maxlength=50
                oninput={ctx.link().callback(GroupsMsg::CreateGroupName)}
            />
            <button
                class="create_group_button"
                onclick={ctx.link().callback(|_| GroupsMsg::CreateGroup)}>
                { "Create Group" }
            </button>
        </div>
        }
    }
    pub fn leave_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Leave Group"} </h1>
            <input
                class="leave_group"
                placeholder="group name"
                maxlength=50
                oninput={ctx.link().callback(GroupsMsg::LeaveGroupName)}
            />
            <button
                class="create_group_button"
                onclick={ctx.link().callback(|_| GroupsMsg::LeaveGroup)}>
                { "Leave Group" }
            </button>
        </div>
        }
    }
    pub fn add_user_to_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Add User"} </h1>
            <input
                class="add_user"
                placeholder="e-mail"
                maxlength=50
                oninput={ctx.link().callback(GroupsMsg::AddUser)}
            />
            //<select id="user_groups" name="groups">
                //<option>{"group 1"}</option>
                //<option>{"group 2"}</option>
            //</select>

            <input
                class="add_user"
                placeholder="group name"
                maxlength=50
                oninput={ctx.link().callback(GroupsMsg::GroupAdd)}
            />
            <button
                class="create_group_button"
                onclick={&ctx.link().callback(|_| GroupsMsg::AddNewUser)}>
                { "Add User" }
            </button>
        </div>
        }
    }
}
