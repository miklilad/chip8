pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub struct Chip8 {
    memory: [u8; 4096],
    // program counter
    pc: u16,
    // used to point at locations in memory
    i: u16,
    delay_timer: u8,
    stack: [u16; 256],
    registers: [u8; 16],
}

impl Chip8 {
    pub fn new(rom: &[u8]) -> Self {
        todo!();
        // Self {
        //     memory: (),
        //     pc: (),
        //     i: (),
        //     delay_timer: (),
        //     stack: (),
        //     registers: (),
        // }
    }
}
