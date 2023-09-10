use yew::prelude::*;
use yew_router::prelude::*;
use frontend::router::{AppRoute, switch};
use stylist::Style;

const STYLE_FILE: &str = include_str!("css/styles.css");

#[function_component(App)]
fn app() -> Html {
    let stylesheet = Style::new(STYLE_FILE).unwrap();

    html! {
        <div class={stylesheet}>
            <BrowserRouter>
                <Switch<AppRoute> render={switch} />
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
