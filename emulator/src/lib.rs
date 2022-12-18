mod components {
    pub mod memory;
    pub mod opcodes;
    pub mod processor;
    pub mod screen;
}

use crate::components::*;
use fluvio_wasm_timer::Delay;
use log::*;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

extern crate console_error_panic_hook;

struct Emulator {
    processor: processor::Processor,
    screen: screen::Screen,
}
impl Emulator {
    const INSTRUCTIONS_PER_SECOND: usize = 100;

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
    console_log::init_with_level(Level::Debug).expect("Failed initializing logger!");

    let mut emulator = Emulator::init();
    emulator.processor.memory.load_fonts();
    emulator
        .processor
        .memory
        .load_rom(include_bytes!("roms/test_opcode.ch8").to_vec());

    debug!("{:#X?}", emulator.processor);
    debug!("{:?}", emulator.screen);

    spawn_local(async move { emulator.start().await });
}
