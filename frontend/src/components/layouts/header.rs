use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::get_url;
use crate::constants::MAIN_MENU_ITEMS;

#[inline_props]
pub fn Header(
    cx: Scope,
    menu_id: Option<i32>,
    active_vid: Option<String>
) -> Element {
    let logo_img = get_url("assets/icons/logo.svg");

    cx.render(rsx! {
        header {
            class: "g-header fixed top-0 left-0 w-full h-7 px-0
                transition-all duration-500 z-[1000]
                flex justify-between box-content items-center bg-purple
                h-[58px] leading-[58px]",
            // logo
            div {
                class: "g-logo ml-[2%] transition-all duration-200 opacity-80 hover:opacity-100",
                Link {
                    to: "/",
                    class: "oic-logo-link w-full h-full flex items-center gap-x-1",
                    span {
                        class: "oic-logo-img w-7 h-7 ml-[2%] bg-no-repeat bg-center bg-contain block",
                        style: "background-image: url({logo_img})",
                    }
                    span {
                        class: "oic-logo-text text-white text-xl break-keep",
                        "爱习网"
                    }
                }
            }
            // 移动端菜单ICON
            i {
                id: "menu-toggle",
                class: "iconfont g-icon-menu icon-menu-right md:hidden absolute
                    t-[22px] r-0 p-2 text-3xl text-white opacity-80 font-[iconfont]
                    hover:opacity-100",
            }

            nav {
                class: "g-nav mr-[2%]",
                ul {
                    class: "flex items-center gap-x-4",
                    MAIN_MENU_ITEMS.iter().map(|item| {
                        rsx! {
                            li {
                                class: "h-full oic-item-{item.vid.as_str()}",
                                Link {
                                    class: "block text-white opacity-70 text-sm px-3 tracking-wide leading-[58px] hover:opacity-100",
                                    to: get_url(item.href.as_str()),
                                    item.name.as_str()
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}