use yew::prelude::*;

pub fn header() -> Html {
    html! {
        <header class="bg-nav">
            <div class="p-1 mx-3 inline-flex items-center">
                <i class="fas fa-bars pr-2 text-white" />
                <h1 class="text-white p-2">
                    { "Logo" }
                </h1>
            </div>
            <div class="p-1 flex flex-row items-center">
                <a
                    href="https://"
                    class="text-white p-2 mr-2 no-underline hidden md:block lg:block"
                >Github</a>

                <img
                    class="inline-block h-8 w-8 rounded-full"
                    src="https://" alt=""
                />
                <a>Adam Wathan</a>
            </div>
        </header>
    }
}