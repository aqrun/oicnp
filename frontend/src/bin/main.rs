use dioxus_fullstack::prelude::*;

fn main() {
    // #[cfg(feature = "web")]
    // dioxus_web::launch(frontend::App);
    LaunchBuilder::new(frontend::App).launch();
}