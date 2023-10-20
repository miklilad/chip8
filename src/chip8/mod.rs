use std::cmp;

use winit::window::Window;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Chip8 {
    pub display: [[u8; WIDTH]; HEIGHT],

    memory: [u8; 4096],
    // program counter
    pc: u16,
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
            display: [[0; WIDTH]; HEIGHT],

            memory,
            pc: 0x200,
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 256],
            v: [0; 16],
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc: usize = self.pc.into();
        let instruction: u16 = ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.pc = self.pc + 2;
        instruction
    }

    /**
     * Clear screen
     */
    fn f00e0(&mut self) {
        self.display = [[0; WIDTH]; HEIGHT];
    }

    /**
     * Jump
     */
    fn f1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /**
     * Set register VX
     */
    fn f6xnn(&mut self, x: u16, nn: u8) {
        self.v[x as usize] = nn;
    }

    /**
     * Add value to register VX
     */
    fn f7xnn(&mut self, x: u16, nn: u8) {
        // TODO: handle overflow somehow
        self.v[x as usize] = self.v[x as usize] + nn;
    }

    /**
     * Set index register I
     */
    fn fannn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /**
     * Display / draw
     */
    fn fdxyn(&mut self, x_register: u16, y_register: u16, n: u8) {
        let x_coord_from = (self.v[x_register as usize] as usize) % WIDTH;
        let y_coord_from = (self.v[y_register as usize] as usize) % HEIGHT;
        let x_coord_to = cmp::min(x_coord_from + 8, WIDTH);
        let y_coord_to = cmp::min(y_coord_from + n as usize, HEIGHT);
        self.v[0xf] = 0;
        for (nth_row, y_coord) in (y_coord_from..y_coord_to).enumerate() {
            let nth_sprite_byte = self.memory[self.i as usize + nth_row];
            let sprite_mask: u8 = 0x80;
            for (i, x_coord) in (x_coord_from..x_coord_to).enumerate() {
                let sprite_pixel = (nth_sprite_byte & (sprite_mask >> i)) >> (7 - i);
                let new_pixel = self.display[y_coord][x_coord] ^ sprite_pixel;
                if (self.display[y_coord][x_coord] & sprite_pixel >= 1) {
                    self.v[0xf] = 1;
                }
                self.display[y_coord][x_coord] = new_pixel;
            }
        }
    }

    fn decode(&mut self, instruction: u16) -> bool {
        let micro_instruction = (instruction & 0xF000) >> 12;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        let mut should_redraw = false;

        match micro_instruction {
            0x0 => match instruction {
                0x00E0 => {
                    self.f00e0();
                    should_redraw = true;
                }
                _ => todo!(),
            },
            0x1 => self.f1nnn(nnn),
            0x2 => {}
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => self.f6xnn(x, nn),
            0x7 => self.f7xnn(x, nn),
            0x8 => {}
            0x9 => {}
            0xA => self.fannn(nnn),
            0xB => {}
            0xC => {}
            0xD => {
                self.fdxyn(x, y, n);
                should_redraw = true;
            }
            0xE => {}
            0xF => {}
            _ => unreachable!(),
        }
        should_redraw
    }

    /**
     * Return true/false if the screen should be redrawn
     */
    pub fn step(&mut self) -> bool {
        let instruction = self.fetch();
        println!("{:04x?}", instruction);
        self.decode(instruction)
    }
}
