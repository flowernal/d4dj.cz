use yew::prelude::*;
use gloo_net::http::{Request, Response};
use gloo::console::log;
use serde::{Serialize, Deserialize};
use crate::components::{
    post::post_form::{Post, PostProps},
    header::Header,
    footer::Footer,
};

#[derive(Serialize, Deserialize, Debug)]
struct PostData {
    title: String,
    body: String,
    user_id: String
}

#[function_component(PostPage)]
pub fn post_page() -> Html {
    let handle_submit = Callback::from(|data: PostProps| {
        let post_data = PostData {
            title: data.title,
            body: data.body,
            user_id: data.user_id
        };

        let serialized_post_data = serde_json::to_string(&post_data).unwrap();

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
