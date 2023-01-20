use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{HtmlInputElement};
use crate::routes::AppRoute;

#[function_component]
pub fn Login() -> Html {
    let history = use_navigator().unwrap();
    let login_handle = {
        Callback::from(move |e: MouseEvent| {
            // let win = web_sys::window().unwrap();
            // // win.a = e;
            // let target = e.target_unchecked_into::<HtmlInputElement>();
            // target.preventDefault();
            // println!("{:?}", target);
            history.push(&AppRoute::Home);
        })
    };

    html! {
        <div
				 class={"page-login-w container mx-auto bg-cover h-screen
								flex flex-1 justify-center items-center flex-col"
				 }
				>
            <div class="main w-full max-w-lg">
                <div class="leading-loose">
                    <form
                        class="mx-w-xl m-4 p-10 bg-white rouded shadow-xl"
                        method="post"
                    >
                        <p class="text-gray-800 font-medium text-center text-lg font-bold">
                            { "Login" }
                        </p>
                        <div class="mt-2">
                            <label class="block text-sm text-gray-00" for="username">
                                { "Username" }
                            </label>
                            <input
                                class="w-full px-5 py-1 text-gray-700 bg-gray-200 rounded"
                                id="username"
                                name="username"
                                type="text"
                                required={true}
                                placeholder="User Name"
                                aria-label="username"
                            />
                        </div>
                        <div class="mt-2">
                            <label
                                class="block text-sm text-gray-600"
                                for="password"
                            >
                                { "Password" }
                            </label>
                            <input
                                class="w-full px-5 py-1 text-gray-700 bg-gray-200
                                rounded"
                                id="password"
                                name="password"
                                type="text"
                                required={true}
                                placeholder="******"
                                aria-label="password"
                            />
                        </div>
                        <div class="mt-4 flex items-center justify-between">
                            <button
                                class="px-4 py-1 text-white font-light tracking-wider
                                bg-gray-900 rounded"
                                type="submit"
                                onclick={login_handle}
                            >
                                { "Login" }
                            </button>
                            <a
                                class="inline-block right-0 align-baseline font-bold
                                text-sm text-500 hover:text-blue-800"
                                href="#"
                            >
                                { "Not registered?" }
                            </a>
                        </div>
                    </form>
                </div>
            </div>
        </div>
    }
}
