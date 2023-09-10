use yew::prelude::*;
use yew_router::prelude::*;
use crate::pages::{
    home::Home,
};

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! { <Home /> },
    }
}
