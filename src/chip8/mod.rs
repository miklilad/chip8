mod stack;

use stack::Stack;
use std::cmp;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub enum Chip8Implementation {
    CosmacVip,
    Modern,
}

pub struct Chip8 {
    pub display: [[u8; WIDTH]; HEIGHT],

    memory: [u8; 4096],
    // program counter
    pc: u16,
    // used to point at locations in memory
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    stack: Stack,
    // general purpose registers
    v: [u8; 16],
    implementation: Chip8Implementation,
}

impl Chip8 {
    pub fn new(rom: &[u8], implementation: Chip8Implementation) -> Self {
        const FONT: [u8; 80] = [
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

        FONT.iter().enumerate().for_each(|(i, byte)| {
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
            stack: Stack::new(),
            v: [0; 16],
            implementation,
        }
    }

    fn fetch(&mut self) -> u16 {
        let pc: usize = self.pc.into();
        let instruction: u16 = ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.pc = self.pc + 2;
        instruction
    }

    /// Clear screen
    fn f00e0(&mut self) {
        self.display = [[0; WIDTH]; HEIGHT];
    }

    /// Return from subroutine
    fn f00ee(&mut self) {
        let return_address = self.stack.pop();
        self.pc = return_address;
    }

    /// Jump
    fn f1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    /// Call subroutine
    fn f2nnn(&mut self, nnn: u16) {
        self.stack.push(nnn);
        self.pc = nnn;
    }

    /// Skips one instruction if the value of VX is equal to NN
    fn f3xnn(&mut self, x: u16, nn: u8) {
        if self.v[x as usize] == nn {
            self.pc += 2;
        }
    }

    /// Skips one instruction if the value of VX is NOT equal to NN
    fn f4xnn(&mut self, x: u16, nn: u8) {
        if self.v[x as usize] != nn {
            self.pc += 2;
        }
    }

    /// Skips one instruction if the value of VX is equal to the value of VY
    fn f5xy0(&mut self, x: u16, y: u16) {
        if self.v[x as usize] == self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Set register VX
    fn f6xnn(&mut self, x: u16, nn: u8) {
        self.v[x as usize] = nn;
    }

    /// Add value to register VX
    fn f7xnn(&mut self, x: u16, nn: u8) {
        self.v[x as usize] = self.v[x as usize].wrapping_add(nn);
    }

    /// Sets VX to the value of  VY
    fn f8xy0(&mut self, x: u16, y: u16) {
        self.v[x as usize] = self.v[y as usize];
    }

    /// VX is set to the bitwise/binary logical disjunction (OR) of VX and VY. VY is not affected.
    fn f8xy1(&mut self, x: u16, y: u16) {
        self.v[x as usize] |= self.v[y as usize];
    }

    /// VX is set to the bitwise/binary logical conjunction (AND) of VX and VY. VY is not affected.
    fn f8xy2(&mut self, x: u16, y: u16) {
        self.v[x as usize] &= self.v[y as usize];
    }

    /// VX is set to the bitwise/binary exclusive OR (XOR) of VX and VY. VY is not affected.
    fn f8xy3(&mut self, x: u16, y: u16) {
        self.v[x as usize] ^= self.v[y as usize];
    }

    /// VX is set to the value of VX plus the value of VY. VY is not affected.
    ///
    /// Unlike 7XNN, this addition will affect the carry flag.
    /// If the result is larger than 255 (and thus overflows the 8-bit register VX),
    /// the flag register VF is set to 1. If it doesn’t overflow, VF is set to 0.
    fn f8xy4(&mut self, x: u16, y: u16) {
        let result = self.v[x as usize].checked_add(self.v[y as usize]);
        self.v[0xF] = match result {
            Some(_) => 0,
            None => 1,
        };
        self.v[x as usize] = self.v[x as usize].wrapping_add(self.v[y as usize]);
    }

    /// Sets VX to the result of VX - VY. Carry flag is set to 0 if the result underflows, else to 1.
    fn f8xy5(&mut self, x: u16, y: u16) {
        let result = self.v[x as usize].checked_sub(self.v[y as usize]);
        self.v[0xF] = match result {
            Some(_) => 1,
            None => 0,
        };
        self.v[x as usize] = self.v[x as usize].wrapping_sub(self.v[y as usize]);
    }

    /// In the CHIP-8 interpreter for the original COSMAC VIP, this instruction did the following:
    /// It put the value of VY into VX, and then shifted the value in VX 1 bit to the right (8XY6) or left (8XYE).
    /// VY was not affected, but the flag register VF would be set to the bit that was shifted out.
    ///
    /// However, starting with CHIP-48 and SUPER-CHIP in the early 1990s,
    /// these instructions were changed so that they shifted VX in place, and ignored the Y completely.
    fn f8xy6(&mut self, x: u16, y: u16) {
        if let Chip8Implementation::CosmacVip = self.implementation {
            self.v[x as usize] = self.v[y as usize];
        }
        self.v[0xf] = self.v[x as usize] & 0x1;
        self.v[x as usize] = self.v[x as usize].wrapping_shr(1);
    }

    /// Sets VX to the result of VY - VX. Carry flag is set to 0 if the result underflows, else to 1.
    fn f8xy7(&mut self, x: u16, y: u16) {
        let result = self.v[y as usize].checked_sub(self.v[x as usize]);
        self.v[0xF] = match result {
            Some(_) => 1,
            None => 0,
        };
        self.v[x as usize] = self.v[y as usize].wrapping_sub(self.v[x as usize]);
    }

    /// Bit shift to the left. Ambiguous: https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#8xy6-and-8xye-shift
    fn f8xye(&mut self, x: u16, y: u16) {
        if let Chip8Implementation::CosmacVip = self.implementation {
            self.v[x as usize] = self.v[y as usize];
        }
        self.v[0xf] = self.v[x as usize] & 0x80;
        self.v[x as usize] = self.v[x as usize].wrapping_shl(1);
    }

    /// Skips one instruction if the value of VX is NOT equal to the value of VY.
    fn f9xy0(&mut self, x: u16, y: u16) {
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }

    /// Sets index register I
    fn fannn(&mut self, nnn: u16) {
        self.i = nnn;
    }

    /// Jump with offset. Jumps to the address NNN plus the value in the register V0.
    ///
    /// Ambiguous instruction! https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#bnnn-jump-with-offset
    fn fbnnn(&mut self, nnn: u16) {
        
    }

    /// Generates a random number, binary ANDs it with the value NN, and puts the result in VX.
    fn fcxnn(&mut self, x: u16, nn: u8) {
        let random = rand::random::<u8>();
        self.v[x as usize] = random & nn;
    }

    /// Display / draw
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
                if self.display[y_coord][x_coord] & sprite_pixel >= 1 {
                    self.v[0xf] = 1;
                }
                self.display[y_coord][x_coord] = new_pixel;
            }
        }
    }

    /// Skips one instruction (increment PC by 2) if the key corresponding to the value in VX is pressed.
    fn fex95(&mut self, x: u16) {}

    /// Skips one instruction (increment PC by 2) if the key corresponding to the value in VX is NOT pressed.
    fn fexa1(&mut self, x: u16) {}

    /// Sets VX to the current value of the delay timer
    fn ffx07(&mut self, x: u16) {
        self.v[x as usize] = self.delay_timer;
    }

    /// Sets the delay timer to the value in VX
    fn ffx15(&mut self, x: u16) {
        self.delay_timer = self.v[x as usize];
    }

    /// Sets the sound timer to the value in VX
    fn ffx18(&mut self, x: u16) {
        self.sound_timer = self.v[x as usize];
    }

    /// The index register I will get the value in VX added to it.
    fn ffx1e(&mut self, x: u16) {
        // TODO set vf if it overflows to 0x1000 range
        self.i = self.i.wrapping_add(x);
    }

    /// This instruction “blocks”; it stops executing instructions
    /// and waits for key input (or loops forever, unless a key is pressed).
    fn ffx0a(&mut self, x: u16) {}

    /// The index register I is set to the address of the hexadecimal character in VX
    fn ffx29(&mut self, x: u16) {}

    /// Binary-coded decimal conversion. https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fx33-binary-coded-decimal-conversion
    fn ffx33(&mut self, x: u16) {}

    /// The value of each variable register from V0 to VX inclusive (if X is 0, then only V0)
    /// will be stored in successive memory addresses, starting with the one that’s stored in I.
    /// V0 will be stored at the address in I, V1 will be stored in I + 1,
    /// and so on, until VX is stored in I + X
    fn ffx55(&mut self, x: u16) {}

    /// Same as FX65, except that it takes the value stored at the memory addresses
    /// and loads them into the variable registers instead.
    fn ffx65(&mut self, x: u16) {}

    fn decode(&mut self, instruction: u16) -> bool {
        let micro_instruction = (instruction & 0xF000) >> 12;
        let x = (instruction & 0x0F00) >> 8;
        let y = (instruction & 0x00F0) >> 4;
        let n = (instruction & 0x000F) as u8;
        let nn = (instruction & 0x00FF) as u8;
        let nnn = instruction & 0x0FFF;

        let mut should_redraw = false;

        let unknown_instruction =
            |instruction: u16| panic!("Unknown instruction {:04x}", instruction);

        match micro_instruction {
            0x0 => match instruction {
                0x00E0 => {
                    self.f00e0();
                    should_redraw = true;
                }
                0x00EE => self.f00ee(),
                _ => unknown_instruction(instruction),
            },
            0x1 => self.f1nnn(nnn),
            0x2 => self.f2nnn(nnn),
            0x3 => self.f3xnn(x, nn),
            0x4 => self.f4xnn(x, nn),
            0x5 => self.f5xy0(x, y),
            0x6 => self.f6xnn(x, nn),
            0x7 => self.f7xnn(x, nn),
            0x8 => match n {
                0x0 => self.f8xy0(x, y),
                0x1 => self.f8xy1(x, y),
                0x2 => self.f8xy2(x, y),
                0x3 => self.f8xy3(x, y),
                0x4 => self.f8xy4(x, y),
                0x5 => self.f8xy5(x, y),
                0x6 => self.f8xy6(x, y),
                0x7 => self.f8xy7(x, y),
                0xe => self.f8xye(x, y),
                _ => unknown_instruction(instruction),
            },
            0x9 => self.f9xy0(x, y),
            0xA => self.fannn(nnn),
            0xB => self.fbnnn(nnn),
            0xC => self.fcxnn(x, nn),
            0xD => {
                self.fdxyn(x, y, n);
                should_redraw = true;
            }
            0xE => match nn {
                0x95 => self.fex95(x),
                0xa1 => self.fexa1(x),
                _ => unknown_instruction(instruction),
            },
            0xF => match nn {
                0x07 => self.ffx07(x),
                0x0a => self.ffx0a(x),
                0x29 => self.ffx29(x),
                0x33 => self.ffx33(x),
                0x55 => self.ffx55(x),
                0x65 => self.ffx65(x),
                _ => unknown_instruction(instruction),
            },
            _ => unreachable!(),
        }
        should_redraw
    }

    /// Returns true/false if the screen should be redrawn
    pub fn step(&mut self) -> bool {
        let instruction = self.fetch();
        self.decode(instruction)
    }
}
