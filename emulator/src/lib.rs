use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() {
    web_sys::window()
        .unwrap()
        .alert_with_message("WebAssembly")
        .unwrap();
}
