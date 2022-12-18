mod components {
    pub mod opcodes;
    pub mod processor;
    pub mod screen;
}

use crate::components::*;
use fluvio_wasm_timer::Delay;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_test::console_log;

extern crate console_error_panic_hook;

struct Emulator {
    processor: processor::Processor,
    screen: screen::Screen,
}
impl Emulator {
    const INSTRUCTIONS_PER_SECOND: usize = 10;

    pub fn init() -> Emulator {
        Emulator {
            processor: processor::Processor::init(),
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
pub fn start() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let mut emulator = Emulator::init();
    emulator.processor.load_fonts();
    emulator
        .processor
        .load_rom(include_bytes!("roms/ibm-logo.ch8").to_vec());

    console_log!("{:#x?}", emulator.processor);
    console_log!("{:?}", emulator.screen);

    spawn_local(async move { emulator.start().await });
}
