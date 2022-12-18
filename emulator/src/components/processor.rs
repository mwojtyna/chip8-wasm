use super::opcodes::*;
use super::screen::Screen;
use array_init::array_init;
use wasm_bindgen::convert::IntoWasmAbi;
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
    pub pc: u16,

    /** Index register - point at locations in memory */
    pub i: u16,

    /** A stack for 16-bit addresses, which is used to call subroutines/functions and return from them */
    pub stack: Vec<u16>,

    /** Delay timer - 8-bit value which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0 */
    pub delay_timer: u8,

    /** Sound timer - 8-bit value which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0 */
    pub sound_timer: u8,

    /** 16 8-bit registers, named V0 to VF. */
    /** VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag */
    pub v: [u8; 16],

    pub memory: [u8; 4096],
    pub gfx: [bool; Screen::WIDTH * Screen::HEIGHT],
}
impl Processor {
    pub fn init() -> Processor {
        Processor {
            pc: ROM_BEGIN_INDEX as u16,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: array_init(|_| 0),
            memory: array_init(|_| 0),
            gfx: array_init(|_| true),
        }
    }

    pub fn cycle(&mut self) {
        // Fetch data
        let instruction = self.fetch();
        self.pc += 2;
        console::log_1(&format!("Instruction: {:#06X}", instruction).into());

        // Decode instruction
        let (first, rest) = self.decode(instruction);
        console::log_1(&format!("First: {:#X}, Rest: {:#05X}", first, rest).into());

        // Execute instruction
        self.execute(first, rest).unwrap_or_else(|err| {
            console::warn_1(&format!("{}", err).into());
        });
    }

    fn fetch(&self) -> u16 {
        let first_half = self.memory[self.pc as usize] as u16;
        let second_half = self.memory[self.pc as usize + 1] as u16;

        (first_half) << 0x8 | second_half
    }
    fn decode(&self, instruction: u16) -> (u16, u16) {
        let first = (instruction & 0xF000) >> 0xC;
        let rest = instruction & 0x0FFF;

        (first, rest)
    }
    fn execute(&mut self, first: u16, rest: u16) -> Result<(), Box<dyn std::error::Error>> {
        let mut not_found = false;

        match first {
            0x0 => match rest {
                0x0E0 => OpCode00E0::execute(self),
                _ => {
                    not_found = true;
                }
            },
            _ => {
                not_found = true;
            }
        }

        if not_found {
            Err(format!("Opcode {:#06X} not recognized!", first << 0xC | rest).into())
        } else {
            Ok(())
        }
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
    use super::Processor;
    use array_init::array_init;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_fetch() {
        // Arrange
        let mut memory: [u8; 4096] = array_init(|_| 0);
        memory[0] = 0xAB;
        memory[1] = 0xCD;
        let pc: u16 = 0x0;

        let mut processor = Processor::init();
        processor.memory = memory;
        processor.pc = pc;

        // Act
        let result = Processor::fetch(&processor);

        // Assert
        let expected = 0xABCD;
        assert_eq!(result, expected, "{:#06X} =/= {:#06X}", result, expected);
    }

    #[wasm_bindgen_test]
    fn test_decode() {
        // Arrange
        let instruction: u16 = 0xABCD;
        let processor = Processor::init();

        // Act
        let result = processor.decode(instruction);

        // Assert
        let expected: (u16, u16) = (0xA, 0xBCD);
        assert_eq!(result, expected, "{:#06X?} =/= {:#06X?}", result, expected);
    }

    #[wasm_bindgen_test]
    fn test_execute_normal() {
        // Arrange
        let first = 0x0;
        let rest = 0x0E0;
        let mut processor = Processor::init();

        // Act
        let _ = processor.execute(first, rest);

        // Assert
        assert_eq!(processor.gfx, array_init(|_| false));
    }

    #[wasm_bindgen_test]
    fn test_execute_not_implemented() {
        // Arrange
        let first = 0xF;
        let rest = 0xFFF;
        let mut processor = Processor::init();

        // Act
        let result = processor.execute(first, rest);

        // Assert
        assert!(result.is_err());
    }
}
