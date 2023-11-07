use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use super::Header;

#[inline_props]
pub fn AppLayout(cx: Scope) -> Element {
    render! {
        div {
            Header {}
            Outlet::<Route> {}
            div {
                "footer"
            }
        }
    }
}