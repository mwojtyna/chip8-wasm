use super::processor::{Compatibility, Processor};
use super::screen::Screen;
use array_init::array_init;
use log::*;

pub struct OpCode00E0 {}
pub struct OpCode00EE {}
pub struct OpCode1NNN {}
pub struct OpCode2NNN {}
pub struct OpCode3XNN {}
pub struct OpCode4XNN {}
pub struct OpCode5XY0 {}
pub struct OpCode6XNN {}
pub struct OpCode7XNN {}
pub struct OpCode8XY0 {}
pub struct OpCode8XY1 {}
pub struct OpCode8XY2 {}
pub struct OpCode8XY3 {}
pub struct OpCode8XY4 {}
pub struct OpCode8XY5 {}
pub struct OpCode8XY6 {}
pub struct OpCode8XY7 {}
pub struct OpCode9XY0 {}
pub struct OpCodeANNN {}
pub struct OpCodeDXYN {}

pub trait OpCode {
    fn execute(processor: &mut Processor, data: &[u16]);
}

impl OpCode for OpCode00E0 {
    fn execute(processor: &mut Processor, _: &[u16]) {
        processor.gfx = array_init(|_| false);
    }
}
impl OpCode for OpCode00EE {
    fn execute(processor: &mut Processor, _: &[u16]) {
        let return_address = processor.stack.pop().expect("Stack is empty!");
        processor.pc = return_address;
    }
}
impl OpCode for OpCode1NNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        processor.pc = data[0];
    }
}
impl OpCode for OpCode2NNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let nnn = data[0];
        processor.stack.push(processor.pc);
        processor.pc = nnn;
    }
}
impl OpCode for OpCode3XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;

        if processor.v[x] == nn {
            processor.pc += 2;
        }
    }
}
impl OpCode for OpCode4XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;

        if processor.v[x] != nn {
            processor.pc += 2;
        }
    }
}
impl OpCode for OpCode5XY0 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;

        if processor.v[x] == processor.v[y] {
            processor.pc += 2;
        }
    }
}
impl OpCode for OpCode6XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;
        processor.v[x] = nn;
    }
}
impl OpCode for OpCode7XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;
        processor.v[x] = processor.v[x].wrapping_add(nn)
    }
}
impl OpCode for OpCode8XY0 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        processor.v[x] = processor.v[y];
    }
}
impl OpCode for OpCode8XY1 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        processor.v[x] |= processor.v[y];
    }
}
impl OpCode for OpCode8XY2 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        processor.v[x] &= processor.v[y];
    }
}
impl OpCode for OpCode8XY3 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        processor.v[x] ^= processor.v[y];
    }
}
impl OpCode for OpCode8XY4 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        let (result, overflow) = processor.v[x].overflowing_add(processor.v[y]);

        processor.v[x] = result;
        processor.v[0xF] = overflow as u8;
    }
}
impl OpCode for OpCode8XY5 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        let (result, overflow) = processor.v[x].overflowing_sub(processor.v[y]);

        processor.v[x] = result;
        processor.v[0xF] = !overflow as u8;
    }
}
impl OpCode for OpCode8XY6 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;

        if processor.compatibility == Compatibility::Original {
            processor.v[x] = processor.v[y];
        }
        processor.v[0xF] = processor.v[x] & 0x1;
        processor.v[x] >>= 0x1;
    }
}
impl OpCode for OpCode8XY7 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        let (result, overflow) = processor.v[y].overflowing_sub(processor.v[x]);

        processor.v[x] = result;
        processor.v[0xF] = !overflow as u8;
    }
}
impl OpCode for OpCode9XY0 {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;

        if processor.v[x] != processor.v[y] {
            processor.pc += 2;
        }
    }
}
impl OpCode for OpCodeANNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        processor.i = data[0];
    }
}
impl OpCode for OpCodeDXYN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let y = data[1] as usize;
        let n = data[2];

        let sprite_x = processor.v[x] as usize % Screen::WIDTH;
        let sprite_y = processor.v[y] as usize % Screen::HEIGHT;
        let height = n as usize;
        let width = 8;
        let mut flipped = false;

        for row in 0..height {
            let sprite = processor.memory.data[processor.i as usize + row];
            debug!("Row {:#02}: {:#010b}", row, sprite);

            for col in 0..width {
                let sprite_bit = (sprite >> (width - 1 - col)) & 0x1;
                let gfx_i = (sprite_y + row) * Screen::WIDTH + (sprite_x + col);

                let prev_gfx = processor.gfx[gfx_i];
                processor.gfx[gfx_i] ^= sprite_bit == 1;

                if prev_gfx && !processor.gfx[gfx_i] {
                    flipped = true;
                }
            }
        }

        processor.v[0xF] = flipped as u8;
        debug!("Flipped: {}", flipped);
    }
}

#[allow(non_snake_case)]
mod tests {
    use crate::components::memory::Memory;

    use super::*;
    use array_init::array_init;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_00E0() {
        // Arrange
        let mut processor = Processor::init();
        processor.gfx = array_init(|_| true);

        // Act
        OpCode00E0::execute(&mut processor, &[]);

        // Assert
        assert_eq!(processor.gfx, array_init(|_| false));
    }

    #[wasm_bindgen_test]
    fn test_00EE() {
        // Arrange
        let mut processor = Processor::init();
        let return_address = 0x201;
        processor.pc = 0x200;
        processor.stack.push(return_address);

        // Act
        OpCode00EE::execute(&mut processor, &[]);

        // Assert
        assert_eq!(
            processor.pc, return_address,
            "PC not equal to return address!"
        );
        assert!(processor.stack.is_empty(), "Stack not popped!");
    }

    #[wasm_bindgen_test]
    fn test_1NNN() {
        // Arrange
        let mut processor = Processor::init();
        let jump_to = 0x123;

        // Act
        OpCode1NNN::execute(&mut processor, &[jump_to]);

        // Assert
        assert_eq!(processor.pc, jump_to);
    }

    #[wasm_bindgen_test]
    fn test_2NNN() {
        // Arrange
        let mut processor = Processor::init();
        let nnn = 0x123;

        // Act
        OpCode2NNN::execute(&mut processor, &[nnn]);

        // Assert
        assert_eq!(
            processor.stack[0],
            Memory::ROM_BEGIN_INDEX as u16,
            "PC not added to stack!"
        );
        assert_eq!(processor.pc, nnn, "PC should be {:06X}!", nnn);
    }

    #[wasm_bindgen_test]
    fn test_3XNN() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let nn = 0x23;
        processor.v[x as usize] = nn;

        // Act
        OpCode3XNN::execute(&mut processor, &[x, nn as u16]);

        // Assert
        assert_eq!(processor.pc, Memory::ROM_BEGIN_INDEX as u16 + 0x2);
    }

    #[wasm_bindgen_test]
    fn test_4XNN() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let nn = 0x23;
        processor.v[x as usize] = 0x0;

        // Act
        OpCode4XNN::execute(&mut processor, &[x, nn as u16]);

        // Assert
        assert_eq!(processor.pc, Memory::ROM_BEGIN_INDEX as u16 + 0x2);
    }

    #[wasm_bindgen_test]
    fn test_5XY0() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x23;

        // Act
        OpCode5XY0::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.pc, Memory::ROM_BEGIN_INDEX as u16 + 0x2);
    }

    #[wasm_bindgen_test]
    fn test_6XNN() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let nn = 0x23;

        // Act
        OpCode6XNN::execute(&mut processor, &[x, nn]);

        // Assert
        assert_eq!(processor.v[x as usize] as u16, nn);
    }

    #[wasm_bindgen_test]
    fn test_7XNN() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let nn = 0x23;
        processor.v[x as usize] = 0x1;

        // Act
        OpCode7XNN::execute(&mut processor, &[x, nn]);

        // Assert
        assert_eq!(processor.v[x as usize] as u16, x + nn);
    }

    #[wasm_bindgen_test]
    fn test_8XY0() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[y as usize] = 0x23;

        // Act
        OpCode8XY0::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x23);
    }

    #[wasm_bindgen_test]
    fn test_8XY1() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY1::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x23 | 0x24);
    }

    #[wasm_bindgen_test]
    fn test_8XY2() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY2::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x23 & 0x24);
    }

    #[wasm_bindgen_test]
    fn test_8XY3() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY3::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x23 ^ 0x24);
    }

    #[wasm_bindgen_test]
    fn test_8XY4_no_overflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY4::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x23 + 0x24, "v[x] should be 0x47");
        assert_eq!(processor.v[0xF], 0x0, "v[0xF] should be 0x0");
    }
    #[wasm_bindgen_test]
    fn test_8XY4_overflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0xFF;
        processor.v[y as usize] = 0x1;

        // Act
        OpCode8XY4::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x0, "v[x] should be 0x0");
        assert_eq!(processor.v[0xF], 0x1, "v[0xF] should be 0x1");
    }

    #[wasm_bindgen_test]
    fn test_8XY5_no_underflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x24;
        processor.v[y as usize] = 0x23;

        // Act
        OpCode8XY5::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x24 - 0x23, "v[x] should be 0x1");
        assert_eq!(processor.v[0xF], 0x1, "v[0xF] should be 0x1");
    }
    #[wasm_bindgen_test]
    fn test_8XY5_underflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY5::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0xFF, "v[x] should be 0xFF");
        assert_eq!(processor.v[0xF], 0x0, "v[0xF] should be 0x0");
    }

    #[wasm_bindgen_test]
    fn test_8XY6_original() {
        // Arrange
        let mut processor = Processor::init_compat(Compatibility::Original);
        let x = 0x1;
        let y = 0x2;
        processor.v[y as usize] = 0x23;

        // Act
        OpCode8XY6::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(
            processor.v[x as usize],
            0x23 >> 1,
            "v[x] should be {:X}",
            0x23 >> 1
        );
        assert_eq!(
            processor.v[0xF],
            0x23 & 0x1,
            "v[0xF] should be {:X}",
            0x23 & 0x1
        );
    }
    #[wasm_bindgen_test]
    fn test_8XY6_new() {
        // Arrange
        let mut processor = Processor::init_compat(Compatibility::New);
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;

        // Act
        OpCode8XY6::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(
            processor.v[x as usize],
            0x23 >> 1,
            "v[x] should be {:X}",
            0x23 >> 1
        );
        assert_eq!(
            processor.v[0xF],
            0x23 & 0x1,
            "v[0xF] should be {:X}",
            0x23 & 0x1
        );
    }

    #[wasm_bindgen_test]
    fn test_8XY7_no_underflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode8XY7::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0x24 - 0x23, "v[x] should be 0x1");
        assert_eq!(processor.v[0xF], 0x1, "v[0xF] should be 0x1");
    }
    #[wasm_bindgen_test]
    fn test_8XY7_underflow() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x24;
        processor.v[y as usize] = 0x23;

        // Act
        OpCode8XY7::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.v[x as usize], 0xFF, "v[x] should be 0xFF");
        assert_eq!(processor.v[0xF], 0x0, "v[0xF] should be 0x0");
    }

    #[wasm_bindgen_test]
    fn test_9XY0() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        processor.v[x as usize] = 0x23;
        processor.v[y as usize] = 0x24;

        // Act
        OpCode9XY0::execute(&mut processor, &[x, y]);

        // Assert
        assert_eq!(processor.pc, Memory::ROM_BEGIN_INDEX as u16 + 0x2);
    }

    #[wasm_bindgen_test]
    fn test_ANNN() {
        // Arrange
        let mut processor = Processor::init();
        let nnn = 0x123;

        // Act
        OpCodeANNN::execute(&mut processor, &[nnn]);

        // Assert
        assert_eq!(processor.i, nnn);
    }

    #[wasm_bindgen_test]
    fn test_DXYN_no_flip() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        let n = 0x3;
        let sprite_x = 0x1;
        let sprite_y = 0x2;
        let gfx_start = sprite_y as usize * Screen::WIDTH + sprite_x as usize;

        processor.v[x as usize] = sprite_x;
        processor.v[y as usize] = sprite_y;
        processor.i = 0x200;
        processor.memory.data[processor.i as usize] = 0b01000001;

        // Act
        OpCodeDXYN::execute(&mut processor, &[x, y, n]);

        // Assert
        assert_eq!(
            processor.gfx[gfx_start..gfx_start + 8],
            [false, true, false, false, false, false, false, true],
            "processor.gfx set incorrectly!"
        );
        assert_eq!(processor.v[0xF], 0x0, "v[0xF] should be 0x0");
    }
    #[wasm_bindgen_test]
    fn test_DXYN_flip() {
        // Arrange
        let mut processor = Processor::init();
        let x = 0x1;
        let y = 0x2;
        let n = 0x3;
        let sprite_x = 0x1;
        let sprite_y = 0x2;
        let gfx_start = sprite_y as usize * Screen::WIDTH + sprite_x as usize;

        processor.v[x as usize] = sprite_x;
        processor.v[y as usize] = sprite_y;
        processor.i = 0x200;
        processor.memory.data[processor.i as usize] = 0b01000001;
        processor.gfx = array_init(|_| true);

        // Act
        OpCodeDXYN::execute(&mut processor, &[x, y, n]);

        // Assert
        assert_eq!(
            processor.gfx[gfx_start..gfx_start + 8],
            [true, false, true, true, true, true, true, false],
            "processor.gfx set incorrectly!"
        );
        assert_eq!(processor.v[0xF], 0x1, "v[0xF] should be 0x1");
    }
}
