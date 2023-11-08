use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::{console, Document};
use crate::get_url;
use crate::constants::MAIN_MENU_ITEMS;

#[inline_props]
pub fn Header(
    cx: Scope,
    menu_id: Option<i32>,
    active_vid: Option<&'static str>
) -> Element {
    let logo_img = get_url("assets/icons/logo.svg");
    let mut menu_visible = use_state(cx, || false);
    let mut nav_style = use_state(cx, || "");

    let menuToggleClickHandle = move |_| {
        let is_visible = !menu_visible;
        let new_nav_style = if is_visible {
            "visibility: visible; height: auto; padding: 1.5rem"
        } else {
            ""
        };
        nav_style.set(new_nav_style);
        menu_visible.set(is_visible);
    };

    cx.render(rsx! {
        header {
            class: "g-header fixed top-0 left-0 w-full px-0
                transition-all duration-500 z-[1000]
                flex justify-between box-content items-center bg-purple
                h-58 leading-58",
            // logo
            div {
                class: "g-logo ml-6 transition-all duration-200 opacity-80 hover:opacity-100",
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
            div {
                class: "oic-header-search-w lg:hidden w-8 h-8 ml-auto flex
                    items-center justify-center hover:cursor-pointer text-white
                    mr-2 opacity-80 hover:opacity-100",
                i {
                    class: "oic-header-search-icon iconfont icon-search text-xl
                        font-[iconfont]",
                }
            }
            div {
                id: "menu-toggle",
                class: "oic-header-menu-toggle w-8 h-8 flex items-center
                    justify-center mr-6 lg:hidden hover:cursor-pointer",
                onclick: menuToggleClickHandle,
                // 移动端菜单ICON
                i {
                    class: "iconfont g-icon-menu icon-menu-right lg:hidden absolute
                        t-[22px] r-0 p-2 text-3xl text-white opacity-80 font-[iconfont]
                        hover:opacity-100",
                }
            }

            nav {
                class: "oic-header-nav mr-6 fixed flex right-0 top-[64px] w-[20rem]
                    items-center bg-purple lg:top-auto px-6 py-0 rounded-lg lg:block
                    h-0 lg:h-auto transition-all duration-200 overflow-hidden invisible
                    lg:bg-transparent lg:relative lg:p-0 lg:rounded-0 lg:w-fit
                    lg:visible lg:overflow-visible",
                style: "{nav_style}",
                ul {
                    class: "flex items-center gap-x-4 flex-col justify-start
                        w-full lg:w-fit lg:justify-center lg:flex-row",
                    MAIN_MENU_ITEMS.iter().map(|item| {
                        let current_vid = String::from(active_vid.unwrap_or(""));
                        let mut active_class = String::from("");

                        if current_vid.eq(item.vid) {
                            active_class = String::from(" oic-active");
                        }

                        rsx! {
                            li {
                                class: "h-full oic-item-{item.vid} flex
                                    w-full lg:w-fit
                                    items-start lg:items-center",
                                Link {
                                    class: "block text-white opacity-70 text-sm 
                                        px-3 tracking-wide leading-58
                                        w-full lg:w-fit lg:leading-58
                                        hover:opacity-100 hover:shadow-md
                                        hover:shadow-purple-300
                                        rounded lg:rounded-none {active_class}",
                                    to: get_url(item.href),
                                    item.name
                                }
                            }
                        }
                    })
                }
            }
        }
    })
}
