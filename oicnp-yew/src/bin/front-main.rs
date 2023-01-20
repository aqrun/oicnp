use oicnp_yew::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::Renderer::<App>::new().render();
    // yew::start_app::<App>();
}
