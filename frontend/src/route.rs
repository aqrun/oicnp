use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::{Serialize, Deserialize};
use crate::pages::{HomePage, PostsPage, Err404};
use crate::components::layouts::AppLayout;

#[derive(Clone, Routable, PartialEq, Eq, Serialize, Deserialize, Debug)]
#[rustfmt::skip]
pub enum Route {
    #[layout(AppLayout)]
        #[route("/")]
        HomePage {},

        #[route("/posts")]
        PostsPage {},
    #[end_layout]

    #[route("/:..segments")]
    Err404 { segments: Vec<String> },
}
