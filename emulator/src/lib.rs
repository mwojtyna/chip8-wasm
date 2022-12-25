mod components {
    pub mod keypad;
    pub mod memory;
    pub mod processor;
    pub mod screen;
}
pub mod opcodes;

use crate::components::*;
use components::processor::Compatibility;
use fluvio_wasm_timer::Delay;
use log::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[derive(Debug)]
struct Emulator {
    processor: processor::Processor,
    screen: screen::Screen,
}
impl Emulator {
    const INSTRUCTIONS_PER_SECOND: usize = 700;

    pub fn init(compatibility: Compatibility) -> Emulator {
        Emulator {
            processor: processor::Processor::init_compat(compatibility),
            screen: screen::Screen::init(),
        }
    }
    pub async fn start(&mut self) {
        loop {
            self.processor.cycle();
            self.screen.update(&self.processor.gfx);

            Delay::new(Duration::from_secs_f32(
                1.0 / Self::INSTRUCTIONS_PER_SECOND as f32,
            ))
            .await
            .expect("Error waiting for delay!");
        }
    }
}

#[wasm_bindgen]
pub fn start(compatibility: &str) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Info).expect("Failed initializing logger!");

    let compatibility_enum = match compatibility {
        "original" => Compatibility::Original,
        "new" => Compatibility::New,
        _ => panic!("Invalid compatibility setting!"),
    };

    let mut emulator = Emulator::init(compatibility_enum);
    emulator.processor.memory.load_fonts();
    emulator
        .processor
        .memory
        .load_rom(include_bytes!("roms/tests/test_suite.ch8").to_vec());

    debug!("{:#X?}", emulator.processor);
    debug!("{:?}", emulator.screen);

    spawn_local(async move { emulator.start().await });
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

    keypad::INSTANCE.lock().unwrap().set_key(key);
}
#[wasm_bindgen]
pub fn on_key_up() {
    keypad::INSTANCE.lock().unwrap().unset_key();
}
