use yew::prelude::*;

#[function_component(Avatar)]
pub fn avatar() -> Html {
    html! {
        <div class="dropdown is-active relative mx-8">
            <button class="h-16 flex items-center text-xl">
                <i class="iconfont icon-OOjs_UI_icon_userAvatar"></i>
                <span class="ml-2">
                    { "Admin" }
                </span>
            </button>
            <div
                class="dropdown-content hidden absolute bg-white
                    top-16 right-0 shadow-md rounded-md py-2"
            >
                <ul class="flex flex-col text-xl text-gray-700">
                    <li>
                        <a class="block py-2 px-8 cursor-pointer
                            hover:bg-gray-200">
                            { "Profile" }
                        </a>
                    </li>
                    <li>
                        <a class="block py-2 px-8 cursor-pointer
                            hover:bg-gray-200">
                            { "Logout" }
                        </a>
                    </li>
                </ul>
            </div>
        </div>
    }
}
