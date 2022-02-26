use crate::pages::group::{Group, GroupMsg};
use crate::CORS_ORIGIN;
use shared_stuff::db_structs::GroupData;
use yew::prelude::*;

impl Group {
    pub fn view_group_id(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h3>{format!("group id is: {}", &ctx.props().id )}</h3>
        }
    }

    pub fn user_customized_view(&self, ctx: &Context<Self>) -> Html {
        match &self.group_data {
            Some(group_data) => {
                html! {
                    <div>
                    { self.view_group_data(ctx, &group_data) }
                    { self.view_add_user_to_group(ctx) }
                    { self.view_leave_group(ctx) }
                    </div>
                }
            }
            None => {
                html! {
                    <p>{format!("This group doesn't exist or you don't have the access to it.")}</p>
                }
            }
        }
    }

    fn view_group_data(&self, _ctx: &Context<Self>, group_data: &GroupData) -> Html {
        let system_url = format!("{}/system/{}", CORS_ORIGIN.to_string(), self.group_id);
        html! {
            <div>
                <p>{format!("group data is:")}</p>
                <li>{format!("Name: {}", group_data.group_name)}</li>
                <li>{format!("Members: {:?}", group_data.members)}</li>
                <li>{format!("Date created: {:?}", group_data.date_created)}</li>
                <li>{format!("Movies watched: {:?}", group_data.movies_watched)}</li>
                <li>
                    {"system url: "}
                    <a href= {system_url.clone()}>{system_url}</a>
                </li>
            </div>
        }
    }

    fn view_add_user_to_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Add User"} </h1>
            <input
                class="add_user"
                placeholder="username"
                maxlength=50
                oninput={ctx.link().callback(GroupMsg::SetAddUser)}
            />
            <button
                class="create_group_button"
                onclick={&ctx.link().callback(|_| GroupMsg::AddUser)}>
                { "Add User" }
            </button>
            <h4> {format!("add user status: {:?}", self.add_user_success)} </h4>
        </div>
        }
    }

    pub fn view_leave_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Leave Group"} </h1>
            <button
                class="create_group_button"
                onclick={ctx.link().callback(|_| GroupMsg::Leave)}>
                { "Leave Group" }
            </button>
        </div>
        }
    }
}
