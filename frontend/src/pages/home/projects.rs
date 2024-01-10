use dioxus::prelude::*;
use super::projects_data::{PROJECTS, ProjectItem};

pub fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "oic-home-projects mt-6 mb-9",
            div {
                class: "oic-inner mx-auto max-w-7xl",
                h3 {
                    class: "text-center text-4xl mb-9 font-normal text-gray-900",
                    "在线文档"
                }
                div {
                    class: "w-full overflow-hidden pb-4",
                    ul {
                        class: "grid grid-cols-1 lg:grid-cols-2 gap-4 mx-6 lg:mx-0",
                        PROJECTS.iter().map(|item| {
                            rsx! (
                                ProjectBlock {
                                    item: item,
                                }
                            )
                        })
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn ProjectBlock<'a>(cx: Scope, item: &'a ProjectItem) -> Element {
    cx.render(rsx! (
        li {
            class: "before:ease-out
                before:transition-[right] before:duration-300
                before:bg-purple-300 before:h-1 before:w-full
                before:content-[''] before:absolute before:bottom-0
                before:right-full before:hover:right-0
                relative rounded-md shadow-sm bg-white p-6
                text-center overflow-hidden
                hover:shadow-md",
            div {
                h4 {
                    class: "mt-4",
                    a {
                        class: "text-2xl leading-6 font-semibold mb-2
                            text-gray-900 hover:text-purple",
                        href: "{item.href.as_str()}",
                        item.name.as_str()
                    }
                }
                p {
                    class: "text-sm leading-6 mt-2",
                    item.desc.as_str()
                }
                div {
                    class: "oic-btns-w flex items-center justify-center
                        mt-4 gap-2",
                    item.tags.iter().map(|n| {
                        rsx! (
                            a {
                                class: "hover:border-purple-300
                                    hover:text-purple-300
                                    px-2 rounded-2xl border-solid
                                    border-gray-300 border",
                                href: n.href.as_str(),
                                target: "_blank",
                                rel: "noreferrer",
                                n.name.as_str()
                            }
                        )
                    })
                }
            }
        }
    ))
}