fn main() {
    #[cfg(feature = "web")]
    dioxus_web::launch(frontend::App);

    #[cfg(feature = "ssr")]
    frontend::app_run();
}
