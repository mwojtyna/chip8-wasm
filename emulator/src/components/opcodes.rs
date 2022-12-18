use super::processor::Processor;

pub trait OpCode {
    fn execute(processor: &mut Processor);
}
pub trait OpCodeWithData {
    fn execute(processor: &mut Processor, data: u16);
}

impl OpCode for OpCode00E0 {
    fn execute(processor: &mut Processor) {
        for pixel in processor.gfx.iter_mut() {
            *pixel = false;
        }
    }
}
impl OpCodeWithData for OpCode1NNN {
    fn execute(processor: &mut Processor, data: u16) {
        processor.pc = data;
    }
}

pub struct OpCode00E0 {}
pub struct OpCode1NNN {}

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
        OpCode1NNN::execute(&mut processor, jump_to);

        // Assert
        assert_eq!(processor.pc, jump_to);
    }
}
