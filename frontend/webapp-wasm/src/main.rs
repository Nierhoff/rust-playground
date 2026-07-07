use webapp_wasm::app::App;

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("App Start");
    yew::Renderer::<App>::new().render();
}
