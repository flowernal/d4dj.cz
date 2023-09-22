use serde_json::json;
use serde::{Serialize, Deserialize};
use yew::prelude::*;
use gloo_net::http::Request;
use crate::components::{
    header::Header,
    footer::Footer,
    login::login_form::{Login, LoginProps}
};

#[derive(Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: Option<String>
}

pub async fn login_req(username: String, password: String) -> LoginResponse {
    let body = json!({
        "username": username,
        "password": password,
    });

    let response = Request::post("https://d4dj.cz/api/users/login")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .unwrap()
        .send()
        .await
        .unwrap()
        .json::<LoginResponse>()
        .await
        .unwrap();

    return response;

}

#[function_component(LoginPage)]
pub fn login() -> Html {
    let handle_submit = Callback::from(|data: LoginProps| {
        wasm_bindgen_futures::spawn_local(async move {
            let response = login_req(data.username, data.password);
            gloo::console::log!(response.await.token);
        });
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
