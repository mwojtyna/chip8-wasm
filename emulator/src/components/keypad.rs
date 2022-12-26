use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref INSTANCE: Mutex<Keypad> = Mutex::new(Keypad::default());
}

#[derive(Default, Debug)]
pub struct Keypad {
    current_key: u8,
    is_key_pressed: bool,
}
impl Keypad {
    pub fn get_current_key(&self) -> u8 {
        self.current_key
    }
    pub fn is_key_pressed(&self) -> bool {
        self.is_key_pressed
    }

    pub fn set_key(&mut self, key: u8) {
        self.current_key = key;
        self.is_key_pressed = true;
    }
    pub fn unset_key(&mut self) {
        self.current_key = 0x0;
        self.is_key_pressed = false;
    }
}
