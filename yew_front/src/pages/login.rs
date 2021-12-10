use crate::utils::{login_request, register_request};
use crate::LOGIN_URL;
use crate::REGISTER_URL;
use serde_json::json;
use shared_stuff::UserInfo;
use wasm_bindgen_futures::spawn_local;
use yew::format::Json;
use yew::format::Text;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::storage::StorageService;

#[derive(Debug)]
pub struct Login {
    pub link: ComponentLink<Self>,
    username: String,
    password: String,
}
pub enum LoginMsg {
    SetUsername(InputData),
    SetPassword(InputData),
    VerifyLogin,
    RegisterUser,
    SetToken(String),
    Noop,
}

impl Component for Login {
    type Message = LoginMsg;
    type Properties = ();
    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use LoginMsg::*;
        match msg {
            Noop => {}
            SetUsername(text) => {
                self.username = text.value;
            }
            SetPassword(text) => {
                self.password = text.value;
            }
            RegisterUser => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                spawn_local(async move {
                    let resp = register_request(&REGISTER_URL, username).await;
                    match resp {
                        Ok(_a) => log::info!("success!"),
                        Err(e) => log::info!("oh no, {:?}", &e),
                    }
                });
            }
            VerifyLogin => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };

                let link_clone = self.link.clone();
                spawn_local(async move {
                    let token = login_request(&LOGIN_URL, username).await;
                    match token {
                        Ok(tok) => link_clone.send_message(LoginMsg::SetToken(tok)),
                        Err(_) => link_clone.send_message(LoginMsg::Noop),
                    }
                });
            }

            SetToken(token) => {
                let mut storage =
                    StorageService::new(Area::Local).expect("problem with local storage");
                let token_text: Text = Json(&token).into();
                storage.store("jwt", token_text);
                log::info!("stored some data");
            }
        }
        log::info!("{:?}", self);
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
    fn view(&self) -> Html {
        html! {        <div>
        { self.register_html() }
        { self.login_html() }
        </div> }
    }
}
