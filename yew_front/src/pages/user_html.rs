use crate::pages::user::{User, UserMsg};
use crate::CORS_ORIGIN;
use yew::prelude::*;

use super::all_groups;

impl User {
    pub fn user_info(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h3> {
                    format!("hello, you: {:?}", ctx.props().username.clone())
                } </h3>
            </div>
        }
        
    }

    pub fn user_customized_view(&self, ctx: &Context<Self>) -> Html {
        if self.authorized {
            html! {
                <div>
                { self.create_group(ctx) }
                { self.display_all_groups(ctx) }
                </div>
            }
        } else {
            html! {
                <p>{format!("Not your page.")}</p>
            }
        }
    }

    pub fn create_group(&self, ctx: &Context<Self>) -> Html {
        html! {
        <div>
            <h1> {"Create Group"} </h1>
            <input
                class="add_group"
                placeholder="group name"
                maxlength=50
                oninput={ctx.link().callback(UserMsg::CreateGroupName)}
            />
            <button
                class="create_group_button"
                onclick={ctx.link().callback(|_| UserMsg::CreateGroup)}>
                { "Create Group" }
            </button>
        </div>
        }
    }
    pub fn display_all_groups(&self, _ctx: &Context<Self>) -> Html {
        let all_groups = self.all_groups.clone();
        // let current_groups = self.current_groups.clone();
        if !all_groups.is_empty() {
            // let callback = ctx.link().callback(GroupsMsg::SetCurrentGroup);
            all_groups
                .iter()
                .map(|group| {
                    let group_url = format!("{}/group/{}", CORS_ORIGIN.to_string(), &group.uuid);
                    let system_url = format!("{}/system/{}", CORS_ORIGIN.to_string(), &group.uuid);
                    html! {
                        <div>
                            <h1> {"Current Groups"} </h1>
                            <li>
                                {group}
                            </li>
                            <p>
                                {"group url: "}
                                <a href= {group_url.clone()}>{group_url}</a>
                            </p>
                            <p>
                                {"system url: "}
                                <a href= {system_url.clone()}>{system_url}</a>
                            </p>
                        </div>
                    }
                })
                .collect::<Html>()
        } else {
            html! {
                <p> {"Join some groups dude"} </p>
            }
        }
    }
}
