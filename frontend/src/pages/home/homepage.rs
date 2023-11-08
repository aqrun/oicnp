use dioxus::prelude::*;
use super::Banner;
use crate::components::layouts::{Header, Footer};
use crate::types::MenuId;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "oic-homePage-container",
            Header {
                active_vid: MenuId::Home.get_vid(),
            }
            main {
                class: "oic-main-sections mx-auto grid",
                Banner {}
            }
            Footer {}
        }
    })
}