pub const WIDTH: u32 = 64;
pub const HEIGHT: u32 = 32;

pub struct Chip8 {
    memory: [u8; 4096],
    // program counter
    pc: usize,
    // used to point at locations in memory
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: [u16; 256],
    // general purpose registers
    v: [u8; 16],
}

impl Chip8 {
    pub fn new(rom: &[u8]) -> Self {
        const font: [u8; 80] = [
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

        let mut memory: [u8; 4096] = [0; 4096];

        font.iter().enumerate().for_each(|(i, byte)| {
            memory[i + 0x050] = *byte;
        });

        rom.iter().enumerate().for_each(|(i, byte)| {
            memory[i + 0x200] = *byte;
        });

        Self {
            memory,
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 256],
            v: [0; 16],
        }
    }

    pub fn step(&self) {
        let instruction: u16 =
            ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16;
            
    }
}
