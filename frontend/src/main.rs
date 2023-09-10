use yew::prelude::*;
use yew_router::prelude::*;
use frontend::router::{AppRoute, switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<AppRoute> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
