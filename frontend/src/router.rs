use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::Home,
    post::PostPage,
};

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/post")]
    Post,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
        AppRoute::Post => html! { <PostPage /> },
    }
}
