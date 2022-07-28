use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="page-not-found-w">
            { "Page not found 404" }
        </div>
    }
}