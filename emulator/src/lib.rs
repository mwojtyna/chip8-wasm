mod components {
    pub mod keypad;
    pub mod memory;
    pub mod processor;
    pub mod screen;
}
pub mod opcodes;

use crate::components::*;
use components::processor::Compatibility;
use log::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Emulator {
    processor: processor::Processor,
    screen: screen::Screen,
}

#[wasm_bindgen]
impl Emulator {
    pub fn init(compatibility: Compatibility) -> Emulator {
        Emulator {
            processor: processor::Processor::init_compat(compatibility),
            screen: screen::Screen::init(),
        }
    }
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.processor.memory.load_rom(rom);
    }

    pub fn cycle(&mut self) {
        self.processor.cycle();
        self.screen.update(&self.processor.gfx);
    }
}

#[wasm_bindgen]
pub fn init() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Info).expect("Failed initializing logger!");
}

#[wasm_bindgen]
pub fn on_key_down(code: &str) {
    let key = match code {
        "Digit1" => 0x1,
        "Digit2" => 0x2,
        "Digit3" => 0x3,
        "Digit4" => 0xC,
        "KeyQ" => 0x4,
        "KeyW" => 0x5,
        "KeyE" => 0x6,
        "KeyR" => 0xD,
        "KeyA" => 0x7,
        "KeyS" => 0x8,
        "KeyD" => 0x9,
        "KeyF" => 0xE,
        "KeyZ" => 0xA,
        "KeyX" => 0x0,
        "KeyC" => 0xB,
        "KeyV" => 0xF,
        _ => 0x0,
    };

    if key != 0x0 {
        keypad::INSTANCE.lock().unwrap().set_key(key);
    }
}
#[wasm_bindgen]
pub fn on_key_up() {
    keypad::INSTANCE.lock().unwrap().unset_key();
}
