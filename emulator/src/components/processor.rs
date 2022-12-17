use web_sys::*;

#[derive(Default, Debug)]
pub struct Processor {
    /** Program counter - points to the current instruction in the memory */
    pc: u16,

    /** Index register - point at locations in memory */
    i: u16,

    /** A stack for 16-bit addresses, which is used to call subroutines/functions and return from them */
    stack: Vec<u16>,

    /** Delay timer - 8-bit value which is decremented at a rate of 60 Hz (60 times per second) until it reaches 0 */
    delay_timer: u8,

    /** Sound timer - 8-bit value which functions like the delay timer, but which also gives off a beeping sound as long as it’s not 0 */
    sound_timer: u8,

    /** 16 8-bit registers, named V0 to VF. */
    /** VF is also used as a flag register; many instructions will set it to either 1 or 0 based on some rule, for example using it as a carry flag */
    v: [u8; 16],
}

impl Processor {
    pub fn init() -> Processor {
        Processor {
            pc: 0x200,
            ..Default::default()
        }
    }

    pub fn cycle(&self) {
        console::log_1(&"Cycle".into());
    }
}
