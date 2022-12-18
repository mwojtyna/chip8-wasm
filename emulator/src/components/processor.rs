use array_init::array_init;
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
const FONT_BEGIN_INDEX: usize = 0x50;
const ROM_BEGIN_INDEX: usize = 0x200;

#[derive(Debug)]
pub struct Processor {
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

    memory: [u8; 4096],
}

impl Processor {
    pub fn init() -> Processor {
        Processor {
            memory: array_init(|_| 0),
            pc: ROM_BEGIN_INDEX as u16,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: array_init(|_| 0),
        }
    }

    pub fn cycle(&mut self) {
        // Fetch data
        let instruction = Self::fetch(self.memory, self.pc);
        self.pc += 2;

        console::log_1(&format!("Instruction: {:#06X}", instruction).into());
    }
    fn fetch(memory: [u8; 4096], pc: u16) -> u16 {
        let first_nibble = memory[pc as usize] as u16;
        let second_nibble = memory[pc as usize + 1] as u16;

        (first_nibble) << 0x8 | second_nibble
    }

    pub fn load_fonts(&mut self) {
        for (i, font) in FONT_SET.iter().enumerate() {
            self.memory[i + FONT_BEGIN_INDEX] = *font;
        }
    }
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, value) in rom.iter().enumerate() {
            self.memory[ROM_BEGIN_INDEX + i] = *value;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::components::processor::Processor;
    use array_init::array_init;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_fetch() {
        // Setup
        let mut memory: [u8; 4096] = array_init(|_| 0);
        memory[0] = 0xab;
        memory[1] = 0xcd;
        let pc: u16 = 0x0;

        // Run
        let result = Processor::fetch(memory, pc);

        // Assert
        let expected = 0xabcd;
        assert_eq!(result, expected, "{:X} =/= {:X}", result, expected);
    }
}
