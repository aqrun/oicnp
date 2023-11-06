use dioxus_fullstack::prelude::*;
use dioxus_router::prelude::*;
use frontend::{App, Route, app_run};

fn main() {
    let use_web_render = false;

    if use_web_render {
        #[cfg(feature = "web")]
        dioxus_web::launch(App);
    } else {
        app_run();
    }
}