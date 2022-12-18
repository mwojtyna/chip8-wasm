use array_init::array_init;

#[derive(Debug)]
pub struct Memory {
    pub data: [u8; 4096],
}
impl Memory {
    pub const FONT_SET: [u8; 80] = [
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
    pub const FONT_BEGIN_INDEX: usize = 0x50;
    pub const ROM_BEGIN_INDEX: usize = 0x200;

    pub fn init() -> Memory {
        Memory {
            data: array_init(|_| 0),
        }
    }

    pub fn load_fonts(&mut self) {
        for (i, font) in Memory::FONT_SET.iter().enumerate() {
            self.data[i + Memory::FONT_BEGIN_INDEX] = *font;
        }
    }
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, value) in rom.iter().enumerate() {
            self.data[Memory::ROM_BEGIN_INDEX + i] = *value;
        }
    }
}

mod tests {
    use super::Memory;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn test_load_fonts() {
        // Arrange
        let mut memory = Memory::init();

        // Act
        memory.load_fonts();

        // Assert
        assert_eq!(
            memory.data
                [Memory::FONT_BEGIN_INDEX..(Memory::FONT_BEGIN_INDEX + Memory::FONT_SET.len())],
            Memory::FONT_SET
        );
    }

    #[wasm_bindgen_test]
    fn test_load_rom() {
        // Arrange
        let mut memory = Memory::init();
        let rom = vec![0xAB, 0xCD];

        // Act
        memory.load_rom(rom.clone());

        // Assert
        assert_eq!(
            memory.data[Memory::ROM_BEGIN_INDEX..(Memory::ROM_BEGIN_INDEX + rom.len())],
            rom
        );
    }
}
