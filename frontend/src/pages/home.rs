use std::{fmt, ops::Deref};

use gloo::console::log;
use yew::prelude::*;
use serde::{Serialize, Deserialize};
use gloo_net::http::Request;
use crate::components::{
    header::Header,
    footer::Footer
};

// [{"id":1,"title":"Pepa Ferko","body":"Miluju Pepa Ferko. Je to borec. Miluju ho.","user_id":1,"created_at":"2023-09-06 15:39:30"}]

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub user_id: u32,
    pub created_at: String
}

#[derive(Properties, PartialEq)]
struct PostProps {
    posts: Vec<Post>,
}

#[function_component(PostList)]
fn post_list(PostProps { posts }: &PostProps) -> Html {
    posts.iter().map(|post| html! {
        <div>
            <p>{format!("{}", post.title)}</p>
            <p>{format!("{}", post.body)}</p>
        </div>
    }).collect()
}

#[function_component(Home)]
pub fn home() -> Html {
    let state = use_state(|| vec![]);
    {
        let state = state.clone();
        use_effect_with_deps(move |_| {
            let state = state.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let response: Vec<Post> = Request::get("https://d4dj.cz/api/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                state.set(response);
            });
            || ()
        }, ());
    }

    html! {
        <div>
            <Header />
            <main>
                <div>
                    <PostList posts={(*state).clone()} />
                </div>
            </main>
            <Footer />
        </div>
    }
}
