use super::memory::Memory;
use super::screen::Screen;
use crate::opcodes::*;
use array_init::array_init;
use log::*;

#[derive(Debug, PartialEq)]
pub enum Compatibility {
    Original,
    New,
}

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

    pub memory: Memory,
    pub gfx: [bool; Screen::WIDTH * Screen::HEIGHT],
    pub compatibility: Compatibility,
}
impl Processor {
    /** Initializes with compatibility for original systems */
    pub fn init() -> Processor {
        info!("Initializing processor with compatibility for original systems");
        Processor {
            pc: Memory::ROM_BEGIN_INDEX as u16,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: array_init(|_| 0),
            memory: Memory::init(),
            gfx: array_init(|_| false),
            compatibility: Compatibility::Original,
        }
    }
    /** Initializes with compatibility for newer systems */
    pub fn init_newer() -> Processor {
        info!("Initializing processor with compatibility for newer systems");
        Processor {
            pc: Memory::ROM_BEGIN_INDEX as u16,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            v: array_init(|_| 0),
            memory: Memory::init(),
            gfx: array_init(|_| false),
            compatibility: Compatibility::New,
        }
    }
    /** Initializes with specified compatibility */
    pub fn init_compat(compatibility: Compatibility) -> Processor {
        match compatibility {
            Compatibility::Original => Processor::init(),
            Compatibility::New => Processor::init_newer(),
        }
    }

    pub fn cycle(&mut self) {
        debug!("==========================");

        // Fetch data
        let instruction = self.fetch();
        self.pc += 2;
        debug!("Instruction: {:#06X}, PC: {:#06X}", instruction, self.pc);

        // Decode instruction
        let (first, rest) = self.decode(instruction);

        // Execute instruction
        self.execute(first, rest).unwrap_or_else(|err| {
            warn!("{}", err);
        });
    }

    fn fetch(&self) -> u16 {
        let first_half = self.memory.data[self.pc as usize] as u16;
        let second_half = self.memory.data[self.pc as usize + 1] as u16;

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
                0x0E0 => {
                    OpCode00E0::execute(self, &[]);

                    debug!("Clear screen");
                }
                0x0EE => {
                    OpCode00EE::execute(self, &[]);

                    debug!("Return from subroutine");
                }
                _ => {
                    not_found = true;
                }
            },
            0x1 => {
                let nnn = rest;
                OpCode1NNN::execute(self, &[nnn]);

                debug!("Jump to {:#06X} -> {:#06X}", rest, self.pc);
            }
            0x2 => {
                let nnn = rest;
                OpCode2NNN::execute(self, &[nnn]);

                debug!(
                    "Call subroutine at {:#06X} -> stack[0]={:#06X}",
                    rest, self.stack[0]
                );
            }
            0x3 => {
                let x = (rest & 0xF00) >> 8;
                let nn = rest & 0x0FF;
                OpCode3XNN::execute(self, &[x, nn]);

                debug!(
                    "Skip next instruction if V{:X} ({:#06X}) == {:#06X}",
                    x, self.v[x as usize], nn
                );
            }
            0x4 => {
                let x = (rest & 0xF00) >> 8;
                let nn = rest & 0x0FF;
                OpCode4XNN::execute(self, &[x, nn]);

                debug!(
                    "Skip next instruction if V{:X} ({:#06X}) != {:#06X}",
                    x, self.v[x as usize], nn
                );
            }
            0x5 => {
                let x = (rest & 0xF00) >> 8;
                let y = (rest & 0x0F0) >> 4;
                OpCode5XY0::execute(self, &[x, y]);

                debug!(
                    "Skip next instruction if V{:X} ({:#06X}) == V{:X} ({:#06X})",
                    x, self.v[x as usize], y, self.v[y as usize]
                );
            }
            0x6 => {
                let x = (rest & 0xF00) >> 8;
                let nn = rest & 0x0FF;
                OpCode6XNN::execute(self, &[x, nn]);

                debug!("Set V{:X} to {:#06X} -> {:#06X}", x, nn, self.v[x as usize]);
            }
            0x7 => {
                let x = (rest & 0xF00) >> 8;
                let nn = rest & 0x0FF;
                OpCode7XNN::execute(self, &[x, nn]);

                debug!("Add {:#06X} to V{:X} -> {:#06X}", nn, x, self.v[x as usize]);
            }
            0x8 => match rest & 0x00F {
                0x0 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY0::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} ({:#06X}) -> {:#06X}",
                        x, y, self.v[y as usize], self.v[x as usize]
                    );
                }
                0x1 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY1::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} | V{:X} -> {:#06X}",
                        x, x, y, self.v[y as usize]
                    );
                }
                0x2 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY2::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} & V{:X} -> {:#06X}",
                        x, x, y, self.v[y as usize]
                    );
                }
                0x3 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY3::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} ^ V{:X} -> {:#06X}",
                        x, x, y, self.v[y as usize]
                    );
                }
                0x4 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY4::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} + V{:X} -> {:#06X}",
                        x, x, y, self.v[x as usize]
                    );
                }
                0x5 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY5::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} - V{:X} -> {:#06X}",
                        x, x, y, self.v[x as usize]
                    );
                }
                0x6 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY6::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} >> 1 -> {:#06X}",
                        x, y, self.v[x as usize]
                    );
                }
                0x7 => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XY7::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} - V{:X} -> {:#06X}",
                        x, y, x, self.v[x as usize]
                    );
                }
                0xE => {
                    let x = (rest & 0xF00) >> 8;
                    let y = (rest & 0x0F0) >> 4;
                    OpCode8XYE::execute(self, &[x, y]);

                    debug!(
                        "Set V{:X} to V{:X} << 1 -> {:#06X}",
                        x, y, self.v[x as usize]
                    );
                }
                _ => {
                    not_found = true;
                }
            },
            0x9 => {
                let x = (rest & 0xF00) >> 8;
                let y = (rest & 0x0F0) >> 4;
                OpCode9XY0::execute(self, &[x, y]);

                debug!(
                    "Skip next instruction if V{:X} ({:#06X}) != V{:X} ({:#06X})",
                    x, self.v[x as usize], y, self.v[y as usize]
                );
            }
            0xA => {
                let nnn = rest;
                OpCodeANNN::execute(self, &[nnn]);

                debug!("Set I to {:#06X} -> {:#06X}", rest, self.i);
            }
            0xB => {
                let nnn = rest;
                if self.compatibility == Compatibility::Original {
                    OpCodeBNNN::execute(self, &[nnn]);

                    debug!(
                        "Jump to {:#06X} + V0 ({:#06X}) -> {:#06X}",
                        nnn, self.v[0], self.pc
                    );
                } else if self.compatibility == Compatibility::New {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeBXNN::execute(self, &[x, nnn]);

                    debug!(
                        "Jump to {:#06X} + V{:X} ({:#06X}) -> {:#06X}",
                        nnn, x, self.v[x as usize], self.pc
                    );
                }
            }
            0xC => {
                let x = (rest & 0xF00) >> 8;
                let nn = rest & 0x0FF;
                OpCodeCXNN::execute(self, &[x, nn]);

                debug!(
                    "Set V{:X} to random byte & {:#06X} -> {:#06X}",
                    x, nn, self.v[x as usize]
                );
            }
            0xD => {
                let x = (rest & 0xF00) >> 8;
                let y = (rest & 0x0F0) >> 4;
                let n = rest & 0x00F;
                OpCodeDXYN::execute(self, &[x, y, n]);

                debug!(
                    "Draw sprite at {}:{} with height {}",
                    self.v[x as usize], self.v[y as usize], n
                );
            }
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
        processor.memory.data = memory;
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
        let result = processor.execute(first, rest);

        // Assert
        assert!(result.is_ok());
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
