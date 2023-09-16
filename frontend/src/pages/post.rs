use yew::prelude::*;
use gloo_net::http::{Request, Response};
use gloo::console::log;
use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::components::{
    post::post_form::{Post, PostProps},
    header::Header,
    footer::Footer,
};

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    title: String,
    body: String,
    user_id: u32
}

#[derive(Serialize, Deserialize)]
struct PostResponse {
    success: bool
}

#[function_component(PostPage)]
pub fn post_page() -> Html {
    let handle_submit = Callback::from(|data: PostProps| {
        let post_data = json!({
            "title": data.title,
            "body": data.body,
            "user_id": data.user_id.parse::<u32>().unwrap()
        });

        wasm_bindgen_futures::spawn_local(async move {
            log!(post_data.to_string());
            let response = Request::post("https://d4dj.cz/api/posts")
                .header("Content-Type", "application/json")
                .body(post_data.to_string())
                .unwrap()
                .send()
                .await
                .unwrap()
                .json::<PostResponse>()
                .await
                .unwrap();

            log!(response.success);
        });
    });

    html! {
        <div class="post">
            <Header />
            <main>
                <h1>{ "Post" }</h1>
                <Post onsubmit={ handle_submit } />
            </main>
            <Footer />
        </div>
    }
}
