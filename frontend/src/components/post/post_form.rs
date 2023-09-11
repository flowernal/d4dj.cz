use std::ops::Deref;

use crate::components::input::text_input::TextInput;
use yew::prelude::*;

#[derive(Default, Clone)]
pub struct PostProps {
    pub title: String,
    pub body: String,
    pub user_id: String
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<PostProps>,
}

#[function_component(Post)]
pub fn post(props: &Props) -> Html {
    let state = use_state(|| PostProps::default());
    
    let clonned_state = state.clone();
    let title_changed = Callback::from(move |title: String| {
        let mut data = clonned_state.deref().clone();
        data.title = title;
        clonned_state.set(data);
    });

    let clonned_state = state.clone();
    let body_changed = Callback::from(move |body: String| {
        let mut data = clonned_state.deref().clone();
        data.body = body;
        clonned_state.set(data);
    });

    let clonned_state = state.clone();
    let userid_changed = Callback::from(move |user_id: String| {
        let mut data = clonned_state.deref().clone();
        data.user_id = user_id;
        clonned_state.set(data);
    });
    
    let form_onsubmit = props.onsubmit.clone();
    let clonned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        form_onsubmit.emit(clonned_state.deref().clone());
    });

    html! {
        <div>
            <form onsubmit={ onsubmit }>
                <TextInput input="title" handle_onchange={ title_changed } />
                <TextInput input="body" handle_onchange={ body_changed } />
                <TextInput input="user_id" handle_onchange={ userid_changed } />
                <button type="submit">{ "Post" }</button>
            </form>
        </div>
    }
}
