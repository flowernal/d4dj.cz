use std::ops::Deref;

use crate::components::input::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct RegisterProps {
    pub username: String,
    pub password: String,
    pub password_confirm: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<RegisterProps>,
}

#[function_component(Register)]
pub fn register(props: &Props) -> Html {
    let state = use_state(|| RegisterProps::default());

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

    let cloned_state = state.clone();
    let password_confirm_changed = Callback::from(move |password_confirm: String| {
        let mut data = cloned_state.deref().clone();
        data.password_confirm = password_confirm;
        cloned_state.set(data);
    });
    
    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        form_onsubmit.emit(cloned_state.deref().clone());
    });

    html! {
        <div>
            <form onsubmit={ onsubmit }>
                <TextInput input="Username" handle_onchange={ username_changed } />
                <TextInput input="Password" handle_onchange={ password_changed } />
                <TextInput input="Confirm Password" handle_onchange={ password_confirm_changed } />
                <button type="submit">{ "Register" }</button>
            </form>
        </div>
    }
}
