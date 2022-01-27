use crate::pages::groups::{Groups, GroupsMsg};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use crate::CORS_ORIGIN;

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
    pub fn display_current_groups(&self, _ctx: &Context<Self>) -> Html {
        let current_groups = self.current_groups.clone();
        // let current_groups = self.current_groups.clone();
        if !current_groups.is_empty() {
            // let callback = ctx.link().callback(GroupsMsg::SetCurrentGroup);
            current_groups
                .iter()
                .map(|group| {
                    let group_url = format!("{}/group/{}", CORS_ORIGIN.to_string(), &group.uuid);
                    let system_url = format!("{}/system/{}", CORS_ORIGIN.to_string(), &group.uuid);
                    html! {
                        <div>
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
