use dioxus::prelude::*;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "this is home page123"
        }
    })
}