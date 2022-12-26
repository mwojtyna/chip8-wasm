use super::memory::Memory;
use super::screen::Screen;
use crate::opcodes::*;
use array_init::array_init;
use log::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::HtmlAudioElement;

#[wasm_bindgen]
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

    audio_element: HtmlAudioElement,
    timer_subtract: f32,

    /** 16 8-bit registers, named V0 to VF. */
    /** VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag */
    pub v: [u8; 16],

    pub compatibility: Compatibility,
    pub memory: Memory,
    pub gfx: [u8; Screen::WIDTH * Screen::HEIGHT],
}
impl Processor {
    /** Initializes with compatibility for original systems */
    pub fn init() -> Processor {
        let processor = Processor {
            pc: Memory::ROM_BEGIN_INDEX,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            audio_element: HtmlAudioElement::new().unwrap(),
            timer_subtract: 0.0,
            v: array_init(|_| 0),
            compatibility: Compatibility::Original,
            memory: Memory::init(),
            gfx: array_init(|_| 0),
        };
        processor.audio_element.set_src("data:audio/wav;base64,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qJrx4KOQjahZxWbcZgztj2c49nKmkId44S71j0c8eV9yDK6uPRzx5X18eDvjvQ6yKo9ZSS6l//8elePK/Lf//IInrOF/FvDoADYAGBMGb7FtErm5MXMlmPAJQVgWta7Zx2go+8xJ0UiCb8LHHdftWyLJE0QIAIsI+UbXu67dZMjmgDGCGl1H+vpF4NSDckSIkk7Vd+sxEhBQMRU8j/12UIRhzSaUdQ+rQU5kGeFxm+hb1oh6pWWmv3uvmReDl0UnvtapVaIzo1jZbf/pD6ElLqSX+rUmOQNpJFa/r+sa4e/pBlAABoAAAAA3CUgShLdGIxsY7AUABPRrgCABdDuQ5GC7DqPQCgbbJUAoRSUj+NIEig0YfyWUho1VBBBA//uQZB4ABZx5zfMakeAAAAmwAAAAF5F3P0w9GtAAACfAAAAAwLhMDmAYWMgVEG1U0FIGCBgXBXAtfMH10000EEEEEECUBYln03TTTdNBDZopopYvrTTdNa325mImNg3TTPV9q3pmY0xoO6bv3r00y+IDGid/9aaaZTGMuj9mpu9Mpio1dXrr5HERTZSmqU36A3CumzN/9Robv/Xx4v9ijkSRSNLQhAWumap82WRSBUqXStV/YcS+XVLnSS+WLDroqArFkMEsAS+eWmrUzrO0oEmE40RlMZ5+ODIkAyKAGUwZ3mVKmcamcJnMW26MRPgUw6j+LkhyHGVGYjSUUKNpuJUQoOIAyDvEyG8S5yfK6dhZc0Tx1KI/gviKL6qvvFs1+bWtaz58uUNnryq6kt5RzOCkPWlVqVX2a/EEBUdU1KrXLf40GoiiFXK///qpoiDXrOgqDR38JB0bw7SoL+ZB9o1RCkQjQ2CBYZKd/+VJxZRRZlqSkKiws0WFxUyCwsKiMy7hUVFhIaCrNQsKkTIsLivwKKigsj8XYlwt/WKi2N4d//uQRCSAAjURNIHpMZBGYiaQPSYyAAABLAAAAAAAACWAAAAApUF/Mg+0aohSIRobBAsMlO//Kk4soosy1JSFRYWaLC4qZBYWFRGZdwqKiwkNBVmoWFSJkWFxX4FFRQWR+LsS4W/rFRb/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////VEFHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAU291bmRib3kuZGUAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMjAwNGh0dHA6Ly93d3cuc291bmRib3kuZGUAAAAAAAAAACU=");

        processor
    }
    /** Initializes with compatibility for newer systems */
    pub fn init_newer() -> Processor {
        let mut processor = Processor::init();
        processor.compatibility = Compatibility::New;

        processor
    }
    /** Initializes with specified compatibility */
    pub fn init_compat(compatibility: Compatibility) -> Processor {
        match compatibility {
            Compatibility::Original => {
                info!("Initializing processor with compatibility for original systems");
                Processor::init()
            }
            Compatibility::New => {
                info!("Initializing processor with compatibility for newer systems");
                Processor::init_newer()
            }
        }
    }

    pub fn cycle(&mut self) {
        debug!("==========================");

        let instruction = self.fetch();
        self.pc += 2;
        debug!("Instruction: {:#06X}, PC: {:#06X}", instruction, self.pc);

        let (first, rest) = self.decode(instruction);

        self.execute(first, rest).unwrap_or_else(|err| {
            warn!("{}", err);
        });

        self.update_timers();
    }
    fn update_timers(&mut self) {
        // We can't do this in a separate thread so we do it this way
        self.timer_subtract += 1.0 / 10.0;

        if self.timer_subtract >= 1.0 {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            #[allow(unused_must_use)]
            if self.sound_timer > 0 {
                self.audio_element.play().expect("Failed to play audio!");
                self.sound_timer -= 1;
            }

            self.timer_subtract = 0.0;
        }
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
    pub fn execute(&mut self, first: u16, rest: u16) -> Result<(), Box<dyn std::error::Error>> {
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
            0xE => match rest & 0x0FF {
                0x9E => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeEX9E::execute(self, &[x]);

                    debug!(
                        "Skip next instruction if key {:#06X} is pressed",
                        self.v[x as usize]
                    );
                }
                0xA1 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeEXA1::execute(self, &[x]);

                    debug!(
                        "Skip next instruction if key {:#06X} is not pressed",
                        self.v[x as usize]
                    );
                }
                _ => {
                    not_found = true;
                }
            },
            0xF => match rest & 0x0FF {
                0x07 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX07::execute(self, &[x]);

                    debug!(
                        "Set V{:X} to delay timer ({:#06X}) -> {:#06X}",
                        x, self.delay_timer, self.v[x as usize]
                    );
                }
                0x0A => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX0A::execute(self, &[x]);

                    debug!("Wait for keypress and store in V{:X}", x);
                }
                0x15 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX15::execute(self, &[x]);

                    debug!(
                        "Set delay timer to V{:X} ({:#06X}) -> {:#06X}",
                        x, self.v[x as usize], self.delay_timer
                    );
                }
                0x18 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX18::execute(self, &[x]);

                    debug!(
                        "Set sound timer to V{:X} ({:#06X}) -> {:#06X}",
                        x, self.v[x as usize], self.sound_timer
                    );
                }
                0x29 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX29::execute(self, &[x]);

                    debug!(
                        "Set I to location of sprite for digit V{:X} ({:#06X}) -> {:#06X}",
                        x, self.v[x as usize], self.i
                    );
                }
                0x33 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX33::execute(self, &[x]);

                    debug!(
                        "Store BCD representation of V{:X} ({:#06X}) in memory at I ({:#06X})",
                        x, self.v[x as usize], self.i
                    );
                }
                0x1E => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX1E::execute(self, &[x]);

                    debug!(
                        "Set I to I + V{:X} ({:#06X}) -> {:#06X}",
                        x, self.v[x as usize], self.i
                    );
                }
                0x55 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX55::execute(self, &[x]);

                    debug!(
						"Store registers V0 through V{:X} in memory starting at location I ({:#06X})",
						x, self.i
					);
                }
                0x65 => {
                    let x = (rest & 0xF00) >> 8;
                    OpCodeFX65::execute(self, &[x]);

                    debug!(
						"Read registers V0 through V{:X} from memory starting at location I ({:#06X})",
						x, self.i
					);
                }
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
