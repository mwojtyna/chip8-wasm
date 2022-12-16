use array_init::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::*;

const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Emulator {
    screen: CanvasRenderingContext2d,

    memory: [u8; 4096],
    /** Program counter - points to the current instruction in the memory */
    pc: u16,
}

impl Emulator {
    pub fn initialize() -> Emulator {
        // Default values
        let mut emulator = Emulator {
            screen: {
                let document = window().unwrap().document().unwrap();
                let canvas_html_element = document
                    .query_selector("canvas")
                    .unwrap()
                    .expect("Canvas not found!");
                let canvas = canvas_html_element
                    .dyn_into::<HtmlCanvasElement>()
                    .expect("Error casting canvas type!");
                canvas
                    .get_context("2d")
                    .unwrap()
                    .expect("Could not get canvas context!")
                    .dyn_into::<CanvasRenderingContext2d>()
                    .expect("Error casting canvas context type!")
            },

            memory: array_init(|_: usize| 0),
            pc: 0x200,
        };

        // Load fonts
        for (i, font) in FONT_SET.iter().enumerate() {
            emulator.memory[i + 0x50] = *font;
        }

        emulator
    }

    pub fn smiley_face(&self) {
        self.screen.begin_path();

        // Draw the outer circle.
        self.screen
            .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the mouth.
        self.screen.move_to(110.0, 75.0);
        self.screen
            .arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI)
            .unwrap();

        // Draw the left eye.
        self.screen.move_to(65.0, 65.0);
        self.screen
            .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        // Draw the right eye.
        self.screen.move_to(95.0, 65.0);
        self.screen
            .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
            .unwrap();

        self.screen.stroke();
    }
}
