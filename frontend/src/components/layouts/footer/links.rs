use dioxus::prelude::*;

pub fn FooterLinks(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "oic-footer-links",
            div {
                class: "oic-inner mx-auto max-w-7xl py-8",
                div {
                    class: "oic-footer-menu-w flex flex-row justify-between",

                    GetHelp {}
                    PolicyAndRules {}
                    Socials {}
                }
            }
        }
    })
}

fn GetHelp(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            h4 {
                class: "text-base font-medium mb-4",
                "获得帮助！"
            }
            ul {
                li {
                    a {
                        href: "",
                        "文档"
                    }
                }
                li {
                    a {
                        href: "",
                        "Rust Forge（贡献者文档）"
                    }
                }
                li {
                    a {
                        href: "",
                        "在用户论坛提问"
                    }
                }
            }
        }
    })
}

fn PolicyAndRules(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            h4 {
                class: "text-base font-medium mb-4",
                "条款与政策"
            }
            ul {
                li {
                    a {
                        href: "",
                        "行为准则"
                    }
                }
                li {
                    a {
                        href: "",
                        "许可证"
                    }
                }
                li {
                    a {
                        href: "",
                        "商标政策和媒体指南"
                    }
                }
                li {
                    a {
                        href: "",
                        "安全问题公示"
                    }
                }
            }
        }
    })
}

fn Socials(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-col",
            h4 {
                class: "text-base font-medium mb-4",
                "社交"
            }
            div {
                class: "flex flex-row flex-wrap items-center",
                a {
                    target: "_blank",
                    href: "",
                    img {
                        src: "",
                        alt: "mastodon"
                    }
                }
                a {
                    target: "_blank",
                    href: "",
                    img {
                        src: "",
                        alt: "twitter"
                    }
                }
                a {
                    target: "_blank",
                    href: "",
                    img {
                        src: "",
                        alt: "youtube"
                    }
                }
                a {
                    target: "_blank",
                    href: "",
                    img {
                        src: "",
                        alt: "discord"
                    }
                }
                a {
                    target: "_blank",
                    href: "",
                    img {
                        src: "",
                        alt: "github"
                    }
                }
            }
        }
    })
}
