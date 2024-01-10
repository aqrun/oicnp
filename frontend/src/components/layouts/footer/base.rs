use dioxus::prelude::*;
use super::bottom::FooterBottom;
use super::links::FooterLinks;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        footer {
            class: "oic-footer",

            FooterLinks {}
            FooterBottom {}
        }
    })
}
