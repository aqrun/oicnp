use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Route> {}
    })
}
