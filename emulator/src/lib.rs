mod chip8;
use wasm_bindgen::prelude::*;

extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn start(width: u32, height: u32) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let emulator = chip8::Emulator::initialize(width, height);
    emulator.test_display();
}
