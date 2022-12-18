use super::processor::Processor;

pub trait OpCode {
    fn execute(processor: &mut Processor);
}
pub trait OpCodeWithData {
    fn execute(processor: &mut Processor, data: &[u16]);
}

impl OpCode for OpCode00E0 {
    fn execute(processor: &mut Processor) {
        for pixel in processor.gfx.iter_mut() {
            *pixel = false;
        }
    }
}
impl OpCodeWithData for OpCode1NNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        processor.pc = data[0];
    }
}
impl OpCodeWithData for OpCode6XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;
        processor.v[x] = nn;
    }
}
#[allow(clippy::expect_fun_call)]
impl OpCodeWithData for OpCode7XNN {
    fn execute(processor: &mut Processor, data: &[u16]) {
        let x = data[0] as usize;
        let nn = data[1] as u8;
        processor.v[x] = processor.v[x]
            .checked_add(nn)
            .expect(&format!("Overflow on V{}!", x));
    }
}

pub struct OpCode00E0 {}
pub struct OpCode1NNN {}
pub struct OpCode6XNN {}
pub struct OpCode7XNN {}

#[allow(non_snake_case)]
mod tests {
    use super::*;
    use array_init::array_init;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_00E0() {
        // Arrange
        let mut processor = Processor::init();
        processor.gfx = array_init(|_| true);

        // Act
        OpCode00E0::execute(&mut processor);

        // Assert
        assert_eq!(processor.gfx, array_init(|_| false));
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
}
