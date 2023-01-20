use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="page-not-found-w">
            { "Page not found 404" }
        </div>
    }
}
