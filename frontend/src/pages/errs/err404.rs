use dioxus::prelude::*;

#[inline_props]
pub fn Err404(cx: Scope, segments: Vec<String>) -> Element {
    cx.render(rsx! {
        div {
            "this is err-404"
        }
    })
}