use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="bg-grey-darkest bg-gray-700 text-white p-2">
            <div class="flex flex-1 mx-auto">
                {"&copy; OICNP 2022"}
            </div>
        </footer>
    }
}