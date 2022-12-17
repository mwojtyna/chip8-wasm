mod components {
    pub mod memory;
    pub mod processor;
    pub mod screen;
}
use components::*;

use fluvio_wasm_timer::Delay;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console;

extern crate console_error_panic_hook;

struct Emulator {
    processor: processor::Processor,
    memory: memory::Memory,
    screen: screen::Screen,
}
impl Emulator {
    pub fn init(width: u32, height: u32) -> Emulator {
        Emulator {
            processor: processor::Processor::init(),
            memory: memory::Memory::init(),
            screen: screen::Screen::init(width, height),
        }
    }

    pub async fn start(&self) {
        loop {
            self.processor.cycle();
            Delay::new(Duration::from_secs_f32(1.0))
                .await
                .expect("Error waiting for delay!");
        }
    }
}

#[wasm_bindgen]
pub fn start(width: u32, height: u32) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let emulator = Emulator::init(width, height);

    console::log_1(&format!("{:?}", emulator.processor).into());
    console::log_1(&format!("{:?}", emulator.memory).into());
    console::log_1(&format!("{:?}", emulator.screen).into());

    emulator.screen.test_display();
    spawn_local(async move { emulator.start().await });
}
