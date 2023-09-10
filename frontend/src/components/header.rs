use crate::router::AppRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div>
                <h1>{ "D4DJCZ XD" }</h1>
            </div>
            <div class="navbar">
                <nav>
                    <Link<AppRoute> to={AppRoute::Home}>{ "Home" }</Link<AppRoute>>
                    <Link<AppRoute> to={AppRoute::Post}>{ "Post" }</Link<AppRoute>>
                </nav>
            </div>
        </header>
    } 
}
