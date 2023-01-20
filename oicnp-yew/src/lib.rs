mod app;
mod pages;
mod routes;
mod components;
mod hooks;

pub use app::*;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// pub fn run_app() -> Result<(), JsValue> {
//     wasm_logger::init(wasm_logger::Config::default());
//     yew::start_app::<App>();
//     Ok(())
// }

