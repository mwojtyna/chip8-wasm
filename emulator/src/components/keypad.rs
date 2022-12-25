use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref INSTANCE: Mutex<Keypad> = Mutex::new(Keypad::default());
}

#[derive(Default, Debug)]
pub struct Keypad {
    current_key: u8,
}
impl Keypad {
    pub fn get_current_key(&self) -> u8 {
        self.current_key
    }

    pub fn set_key(&mut self, key: u8) {
        self.current_key = key;
    }
    pub fn unset_key(&mut self) {
        self.current_key = 0x0;
    }
}
