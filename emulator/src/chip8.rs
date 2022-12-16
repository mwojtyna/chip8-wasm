use array_init::*;
use wasm_bindgen::JsCast;
use web_sys::*;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

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
const FONT_BEGIN_INDEX: usize = 0x50;

#[derive(Debug)]
pub struct Emulator {
    display: CanvasRenderingContext2d,
    memory: [u8; 4096],

    /** Program counter - points to the current instruction in the memory */
    pc: u16,

    /** Index register - point at locations in memory */
    i: u16,

    /** A stack for 16-bit addresses, which is used to call subroutines/functions and return from them */
    stack: Vec<u16>,

    /** Delay timer - 8-bit value which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0 */
    delay_timer: u8,

    /** Sound timer - 8-bit value which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0 */
    sound_timer: u8,

    /** 16 8-bit registers, named V0 to VF. */
    /** VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag */
    v: [u8; 16],
}

impl Emulator {
    pub fn initialize() -> Emulator {
        // Default values
        let mut emulator = Emulator {
            display: {
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

            memory: array_init(|_| 0),
            pc: 0x200,
            i: 0,
            stack: Vec::<u16>::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: array_init(|_| 0),
        };

        // Load fonts
        for (i, font) in FONT_SET.iter().enumerate() {
            emulator.memory[i + FONT_BEGIN_INDEX] = *font;
        }

        console::log_1(&format!("{:?}", emulator).into());

        emulator
    }

    pub fn test_display(&self) {
        for x in 0..DISPLAY_WIDTH {
            for y in 0..DISPLAY_HEIGHT {
                let color = (x as f32) * (y as f32)
                    / ((DISPLAY_WIDTH - 1) as f32 * (DISPLAY_HEIGHT - 1) as f32)
                    * 255.0;

                self.display
                    .set_fill_style(&format!("rgb({}, {}, {})", color, color, color).into());
                self.display.fill_rect(x as f64, y as f64, 1.0, 1.0);
            }
        }
    }
}
