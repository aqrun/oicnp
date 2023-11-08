use dioxus::prelude::*;

pub fn Footer (cx: Scope) -> Element {
    let date = chrono::Local::now();
    let year = format!("{}", date.format("%Y"));

    cx.render(rsx! {
        footer {
            class: "oic-footer min-h-[2.5rem] py-8 text-center border-t
                border-solid border-slate-100 font-light text-gray-400",
            section {
                class: "text-sm",
                "爱习网 © 2014-{year.as_str()}"
            }
            section {
                class: "text-sm",
                "Powered by ",
                a {
                    href: "https://www.rust-lang.org/",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "hover:text-purple-300",
                    "Rust"
                }
                span {
                    class: "mx-1",
                    "|"
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "hover:text-purple-300",
                    "Dioxus"
                }
                span {
                    class: "mx-1",
                    "|"
                }
                a {
                    href: "https://tailwindcss.com/",
                    target: "_blank",
                    rel: "noreferrer",
                    class: "hover:text-purple-300",
                    "Tailwind"
                }
            }
        }
    })
}