use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus_fullstack::prelude::*;
use crate::Route;

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        Router::<Route> {}
    })
}

pub fn app_run() {
    let config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();

    #[cfg(features = "ssr")]
    config.incremental(
        IncrementalRendererConfig::default()
            .invalidate_after(std::time::Duration::from_secs(120)),
    ).launch();

    #[cfg(not(features = "ssr"))]
    config.launch();
}