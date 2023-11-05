use dioxus::prelude::*;

pub fn PostsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "this is post-page"
        }
    })
}