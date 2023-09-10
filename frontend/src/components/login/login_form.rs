use std::ops::Deref;

use crate::components::input::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct LoginProps {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<LoginProps>,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    let state = use_state(|| LoginProps::default());

    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        let mut data = cloned_state.deref().clone();
        data.username = username;
        cloned_state.set(data);
    });

    let cloned_state = state.clone();
    let password_changed = Callback::from(move |password: String| {
        let mut data = cloned_state.deref().clone();
        data.password = password;
        cloned_state.set(data);
    });
    
    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        form_onsubmit.emit(cloned_state.deref().clone());
    });

    html! {
        <div>
            <form onsubmit={ onsubmit }>
                <TextInput input="Username" handle_onchange={ username_changed } />
                <TextInput input="Password" handle_onchange={ password_changed } />
                <button type="submit">{"Login"}</button>
            </form>
        </div>
    }
}
