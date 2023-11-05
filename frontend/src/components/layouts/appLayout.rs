use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[inline_props]
pub fn AppLayout(cx: Scope) -> Element {
    render! {
        div {
            div {
                "Nav"

                ul {
                    li {
                        Link {
                            to: Route::HomePage {},
                            "go Home"
                        }
                    }
                    li {
                        Link {
                            to: Route::PostsPage {},
                            "go posts page"
                        }
                    }
                }
            }
            Outlet::<Route> {}
            div {
                "footer"
            }
        }
    }
}