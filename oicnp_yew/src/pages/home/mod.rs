use yew::prelude::*;
use crate::components::layouts::{Footer};

#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <div class="page-home-w">
            { "Home page" }
        </div>
    }
}