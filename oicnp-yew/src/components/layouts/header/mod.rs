mod avatar;
mod nav_item;

use yew::prelude::*;
use self::avatar::Avatar;
use self::nav_item::NavItem;

#[function_component]
pub fn Header() -> Html {
    html! {
        <div
            class="layout-header-w flex place-content-between items-center
                h-full"
        >
            <div
                class="page-logo-w text-white text-center text-2xl font-bold
                    w-80"
            >
                { "TEST APP" }
            </div>
            <div class="page-logo-extra flex text-white">
                <div class="navbar-menu">
                    <div class="navbar-start flex">
                        <NavItem label="Home" />
                        <NavItem label="Documentation" />
                        <NavItem label="Jobs" />
                        <NavItem label="About" />
                    </div>
                </div>
                <Avatar />
                

            </div>
        </div>
    }
}

