use super::processor::Processor;

pub trait OpCode {
    fn execute(processor: &mut Processor);
}
impl OpCode for OpCode00E0 {
    fn execute(processor: &mut Processor) {
        for pixel in processor.gfx.iter_mut() {
            *pixel = false;
        }
    }
}

pub struct OpCode00E0 {}

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
}
