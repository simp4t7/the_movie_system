use crate::utils::send_user_info;
use crate::LOGIN_URL;
use crate::REGISTER_URL;
use shared_stuff::UserInfo;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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
                    let resp = send_user_info(REGISTER_URL, username).await;
                    match resp {
                        Ok(a) => log::info!("success!"),
                        Err(e) => log::info!("oh no, {:?}", &e),
                    }
                });
            }
            VerifyLogin => {
                let username = UserInfo {
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                spawn_local(async move {
                    let resp = send_user_info(LOGIN_URL, username).await;
                    match resp {
                        Ok(a) => log::info!("success!"),
                        Err(e) => log::info!("oh no, {:?}", &e),
                    }
                });
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
