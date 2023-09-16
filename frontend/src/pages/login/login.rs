use serde_json::json;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use yewdux::prelude::*;
use gloo_net::http::Request;
use crate::components::{
    header::Header,
    footer::Footer,
    login::login_form::{Login, LoginProps}
};

#[derive(Deserialize)]
pub struct LoginResponse {
    pub id: u32,
    pub username: String,
}

pub async fn login_req(username: String, password: String) -> LoginResponse {
    let body = json!({
        "username": username,
        "password": password,
    });

    todo!();
}

#[function_component(LoginPage)]
pub fn login() -> Html {
    let handle_submit = Callback::from(|data: LoginProps| {
        print!("cs");
    });
    html! {
        <div>
            <Header />
            <main>
                <Login onsubmit={ handle_submit }/>
            </main>
            <Footer />
        </div>
    }
}
