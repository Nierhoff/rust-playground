pub mod app;
pub mod components;
pub mod error;
pub mod hooks;
pub mod services;
pub mod types;

use app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
