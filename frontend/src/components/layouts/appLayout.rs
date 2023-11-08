use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[inline_props]
pub fn AppLayout(cx: Scope) -> Element {
    render! {
        div {
            class: "oic-app-layout pt-58",
            Outlet::<Route> {}
        }
    }
}