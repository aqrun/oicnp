fn main() {
    let use_web_render = false;

    if use_web_render {
        #[cfg(feature = "web")]
        dioxus_web::launch(frontend::App);
    } else {
        frontend::app_run();
    }
}