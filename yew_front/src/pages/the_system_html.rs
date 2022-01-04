use crate::pages::the_system::TheSystem;
use yew::prelude::*;

impl TheSystem {
    pub fn system_ready_status(&self, ctx: &Context<Self>) -> Html {
        if self.ready_status {
            html! {
                <div>
                    <p> {"system ready"} </p>
                </div>

            }
        } else {
            html! {

                <div>
                <p> {"system is not ready"} </p>
                <p> {"users not ready: "} </p>
                {self.users_not_ready
                    .iter()
                    .map(|user| {
                        html! {
                                <p> {user} </p>
                        }
                    })
                    .collect::<Html>()}
                </div>
            }
        }
    }
}
