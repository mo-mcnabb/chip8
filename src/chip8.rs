extern crate rand;

use crate::pixel::Pixel;
use crate::renderer::Renderer;
use rand::prelude::Rng;

const DEFAULT_CHIP8_PIXEL_HEIGHT: u32 = 64;
const DEFAULT_CHIP8_PIXEL_WIDTH: u32 = 32;

pub struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: Vec<u16>,
    instruction_pointer: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub vram: Vec<Vec<Pixel>>,
    pub vram_changed: bool,
    pub vram_scale: usize,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            stack: Vec::new(),
            instruction_pointer: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
            vram: Vec::new(),
            vram_changed: false,
            vram_scale: 1,
        };

        chip8.load_sprites_into_memory();
        chip8
    }

    pub fn load_sprites_into_memory(&mut self) {
        //0b1111_0000, 0b0101_0000, 0b0101_00000
        let built_in_sprites: [u8; 80] = [
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

        built_in_sprites
            .iter()
            .enumerate()
            .for_each(|(index, byte)| self.memory[index] = *byte);
    }

    pub fn initialize_pixels(&mut self, height: u32, width: u32) -> Result<(), String> {
        if height % DEFAULT_CHIP8_PIXEL_HEIGHT != 0 {
            return Err(format!("Window height is not evenly divisible by default height. Window height: {}, default height: {}", height, DEFAULT_CHIP8_PIXEL_HEIGHT));
        }

        if width % DEFAULT_CHIP8_PIXEL_WIDTH != 0 {
            return Err(format!("Window width is not evenly divisible by default width. Window width: {}, default width: {}", width, DEFAULT_CHIP8_PIXEL_WIDTH));
        }

        let height_scale = height / DEFAULT_CHIP8_PIXEL_HEIGHT;
        let width_scale = width / DEFAULT_CHIP8_PIXEL_WIDTH;

        if height_scale != width_scale {
            return Err(format!("Window width scale and window height scale do not match. Width scale: {}, height scale: {}", width_scale, height_scale));
        }

        //arbitrarily chose height scale, it and width_scale should be equal here
        // if theyre not, we're screwed!
        self.vram_scale = height_scale as usize;

        // 64 x 32 pixels
        for y_location in 0..DEFAULT_CHIP8_PIXEL_WIDTH {
            let mut row: Vec<Pixel> = Vec::new();
            for x_location in 0..DEFAULT_CHIP8_PIXEL_HEIGHT {
                //let on = (x_location + y_location) % 2 == 0;
                row.push(Pixel::new(x_location, y_location, false));
            }
            self.vram.push(row);
        }

        Ok(())
    }

    pub fn set_register_value(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value;
    }

    pub fn handle_instruction(&mut self, instruction: u16) {
        match instruction >> 12 {
            0x00E0 => {
                todo!("clear diplay");
            }
            0x00EE => {
                todo!("return");
            }
            0x1 => {
                todo!("goto NNN");
            }
            0x2 => {
                todo!("call subroutine at NNN");
            }
            0x3 => {
                todo!("conditional, 3XNN: skips next instruction if Vx = NN");
            }
            0x4 => {
                todo!("conditional, 4XNN: skips next instruction if Vx != NN");
            }
            0x5 => {
                todo!("conditional, 5XY0: skips next instruction if Vx == Vy");
            }
            0x6 => {
                //todo!("6XNN: sets Vx to NN");
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                self.registers[register_index] = (instruction & 0x00FF) as u8;
            }
            0x7 => {
                //todo!("7XNN: adds NN to Vx (carry flag not changed)");
                let register_index = ((instruction & 0x0F00) >> 8) as usize;
                self.registers[register_index] =
                    self.registers[register_index] + ((instruction & 0x00FF) as u8);
            }
            0x8 => match instruction & 0x000F {
                0x0 => {
                    //todo!("8XY0: sets Vx to Vy");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                    self.registers[register_x_index] = self.registers[register_y_index];
                }
                0x1 => {
                    //todo!("8XY1: sets Vx to Vx | Vy. Vx = Vx | Vy");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                    self.registers[register_x_index] =
                        self.registers[register_x_index] | self.registers[register_y_index];
                }
                0x2 => {
                    //todo!("8XY2: sets Vx to Vx & Vy. Vx = Vx & Vy");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                    self.registers[register_x_index] =
                        self.registers[register_x_index] & self.registers[register_y_index];
                }
                0x3 => {
                    //todo!("8XY3: sets Vx to Vx xor Vy. Vx = Vx ^ Vy");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                    self.registers[register_x_index] =
                        self.registers[register_x_index] ^ self.registers[register_y_index];
                }
                0x4 => {
                    //todo!("8XY4: Adds Vy to Vx. VF(carry flag) is set to 1 when there's an overflow, and to 0 when there is not");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                    match self.registers[register_x_index]
                        .checked_add(self.registers[register_y_index])
                    {
                        Some(output) => {
                            self.registers[register_x_index] = output;
                            self.registers[0x0F] = 0;
                        }
                        None => {
                            self.registers[register_x_index] = self.registers[register_x_index]
                                .wrapping_add(self.registers[register_y_index]);
                            self.registers[0x0F] = 1;
                        }
                    }
                }
                0x5 => {
                    //todo!("8XY5: Vy is subtracted from Vx. VF (carry flag) is set to 0 when there is an underflow, and 1 when there is not. (VF = 1 if Vx >= Vy and 0 if not)");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                    match self.registers[register_x_index]
                        .checked_sub(self.registers[register_y_index])
                    {
                        Some(output) => {
                            self.registers[register_x_index] = output;
                            self.registers[0x0F] = 1;
                        }
                        None => {
                            self.registers[register_x_index] = self.registers[register_x_index]
                                .wrapping_sub(self.registers[register_y_index]);
                            self.registers[0x0F] = 0;
                        }
                    }
                }
                0x6 => {
                    //                    todo!("8XY6: stores to least significant bit of Vx in VF and then shifts Vx to the right by 1. Vx = Vx >> 1");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;

                    let least_sig_bit = self.registers[register_x_index] & 0x01;
                    self.registers[0x0F] = least_sig_bit;
                    self.registers[register_x_index] = self.registers[register_x_index] >> 1;
                }
                0x7 => {
                    //todo!("8XY7: sets Vx to Vy minus Vx. Vf is set to 0 when there is an underflow, and 1 when there is not. i.e. VF = 1 when Vy >= Vx");

                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_y_index = ((instruction & 0x00F0) >> 4) as usize;

                    match self.registers[register_y_index]
                        .checked_sub(self.registers[register_x_index])
                    {
                        Some(output) => {
                            self.registers[register_x_index] = output;
                            self.registers[0x0F] = 1;
                        }
                        None => {
                            self.registers[register_x_index] = self.registers[register_y_index]
                                .wrapping_sub(self.registers[register_x_index]);
                            self.registers[0x0F] = 0;
                        }
                    }
                }
                0xE => {
                    //todo!("8XYE: stores the most significant bit in VF and shifts VX to the left by 1. Vx = Vx << 1");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let most_sig_bit = (self.registers[register_x_index] & 0b1000_0000) >> 7;
                    self.registers[0x0F] = most_sig_bit;
                    self.registers[register_x_index] = self.registers[register_x_index] << 1;
                }
                _ => {
                    todo!("invalid opcod3")
                }
            },
            0x9 => {
                todo!("9XY0: skips the next instruction if Vx != Vy");
            }
            0xA => {
                //todo!("ANNN: Sets the I(instruction) address to NNN");
                let value = instruction & 0x0FFF;
                self.instruction_pointer = value;
            }
            0xB => {
                //todo!("BNNN: jumps to the address NNN plus V0. PC(program counter) = V0 + NNN");
                let value = instruction & 0x0FFF;
                self.program_counter = self.registers[0x00] as u16 + value;
            }
            0xC => {
                //todo!("CXNN: sets Vx to the result of a bitwise and operation on a random number (typically 0 to 255) and NN. Vx = rand() & NN");
                let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                let num = (instruction & 0x00FF) as u8;
                let random_number: u8 = rand::thread_rng().gen();
                self.registers[register_x_index] = random_number & num;
            }
            0xD => {
                //todo!("DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen");
                let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                let register_y_index = ((instruction & 0x00F0) >> 4) as usize;
                let pixel_height = (instruction & 0x000F) as usize;
                let pixel_width = 8; //pixel width is always 8
                                     //let ip = self.instruction_pointer as usize;
                let ip = 60 as usize;

                for row_offset in 0..pixel_height {
                    let row_byte = self.memory[ip + row_offset];
                    println!("row byte: {:#b}", row_byte);
                    for column_offset in 0..8 {
                        let bit_shift_amount = 7 - column_offset;
                        println!("bit shift amount: {bit_shift_amount}");
                        let and_val = 0b1000_0000 >> column_offset;
                        let pixel_val = ((row_byte & and_val) >> bit_shift_amount) == 1;
                        println!("pixel_val = {pixel_val}");
                        let x_location = self.registers[register_x_index];
                        let y_location = self.registers[register_y_index];

                        println!("xl: {x_location}");
                        println!("yl: {y_location}");

                        self.vram[(y_location as usize + row_offset)]
                            [(x_location as usize + column_offset)]
                            .set(pixel_val);
                    }
                }

                self.vram_changed = true;
            }
            0xE => match instruction & 0x00F0 {
                0x0090 => {
                    todo!("EX9E: skips the next instruction if the key stored in Vx is pressed (usually the next instruction is a jump to skip a code block). if(key() == Vx)");
                }
                0x00A0 => {
                    todo!("EXA1: skips the next instruction if the key stored in Vx is not pressed (usually the next instruction is a jump to skip a code block. if (key() != Vx))");
                }
                _ => {
                    todo!("invalid opcode");
                }
            },
            0xF => match instruction & 0x00FF {
                0x0007 => {
                    //todo!("FX07: sets vx to the value of the delay timer. Vx = get_delay()");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    self.registers[register_x_index] = self.delay_timer;
                }
                0x000A => {
                    todo!("FX0A: A key press is awaited, and then stored in Vx (blocking operation, all instruction halted until next key event. probably a loop?)");
                }
                0x0015 => {
                    //todo!("FX15: sets the delay timer to Vx. delay_timer(Vx)");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    self.delay_timer = self.registers[register_x_index];
                }
                0x001E => {
                    //todo!("FX1E: Adds Vx to I. VF is not affected. I = I + Vx");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    self.instruction_pointer =
                        self.instruction_pointer + self.registers[register_x_index] as u16
                }
                0x0029 => {
                    todo!("FX29: sets I to the location of the sprite for the character in Vx. characters 0-F in hex are represented by a 4x5 font. I = sprite_addr[Vx]");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                }
                0x0033 => {
                    //todo!("FX33: stores the binary-codeddecimal representation of Vx, with the hundreds digit in memory at location I, the tens digit at location I+1, and the ones digit at locaion I + 2");
                    let register_x_index = ((instruction & 0x0F00) >> 8) as usize;
                    let register_x_val = self.registers[register_x_index];
                    let hundreds = (register_x_val / 100) % 10;
                    let tens = (register_x_val / 10) % 10;
                    let ones = register_x_val % 10;
                    let instruction_pointer = self.instruction_pointer as usize;

                    self.memory[instruction_pointer] = hundreds;
                    self.memory[instruction_pointer + 1] = tens;
                    self.memory[instruction_pointer + 2] = ones;
                }
                0x0055 => {
                    //todo!("FX55: stores from V0 to Vx (including Vx) in memory, starting at address I. the offset from I is increased by 1 for each value written, but I itself is left unmodified. reg_dum(Vx, &I)");
                    //+1 bc zero index
                    let max_register = (((instruction & 0x0F00) >> 8) + 1) as usize;
                    let instruction_pointer = self.instruction_pointer as usize;
                    self.registers
                        .iter()
                        .take(max_register)
                        .enumerate()
                        .for_each(|(index, register)| {
                            self.memory[instruction_pointer + index] = *register;
                        });
                }
                0x0065 => {
                    //todo!("FX65: Fills from V0 to Vx (including Vx) with values from memory, starting at address I. the offset from I is increased by 1 for each value read, but I remains umodified.");
                    let max_register = (((instruction & 0x0F00) >> 8) + 1) as usize;
                    let ip = self.instruction_pointer as usize;
                    let mem_slice = &self.memory[ip..ip + max_register];

                    mem_slice.iter().enumerate().for_each(|(index, mem_val)| {
                        self.registers[index] = *mem_val;
                    });
                }
                _ => {}
            },
            _ => (),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chip8_initialization() {
        let chip8 = Chip8::new();
        let empty_vec: Vec<u16> = Vec::new();

        assert_eq!(4096, chip8.memory.len());
        assert_eq!(16, chip8.registers.len());
        assert_eq!(empty_vec, chip8.stack);
        assert_eq!(0, chip8.instruction_pointer);
        assert_eq!(0, chip8.program_counter);
        assert_eq!(0, chip8.delay_timer);
        assert_eq!(0, chip8.sound_timer);
    }

    #[test]
    fn load_sprites_into_memory_test() {
        let chip8 = Chip8::new();
        // **val because for some reason it is a &&u8 value lol
        let non_empty_memory_values = chip8.memory.iter().filter(|val| **val != 0).count();

        assert_eq!(80, non_empty_memory_values);
    }

    #[test]
    fn opcode_6xnn_test() {
        let instruction = 0x638F;
        let mut chip8 = Chip8::new();

        chip8.handle_instruction(instruction);
        assert_eq!(0x8F, chip8.registers[3]);

        let instruction = 0x6E0A;
        chip8.handle_instruction(instruction);
        assert_eq!(0x0A, chip8.registers[0x0E]);
    }

    #[test]
    fn opcode_7xnn_test() {
        let instruction = 0x7843;
        let mut chip8 = Chip8::new();
        chip8.registers[8] = 10;
        let expected_result = 0x43 + 10;

        chip8.handle_instruction(instruction);
        assert_eq!(expected_result, chip8.registers[8]);
    }

    #[test]
    fn opcode_8xy0_test() {
        let instruction = 0x8B30;
        let mut chip8 = Chip8::new();
        chip8.registers[0x0B] = 23;
        chip8.registers[3] = 89;

        chip8.handle_instruction(instruction);
        assert_eq!(89, chip8.registers[0x0B]);
    }

    #[test]
    fn opcode_8xy1_test() {
        let instruction = 0x85D1;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 0x32;
        chip8.registers[0x0D] = 0xA0;

        chip8.handle_instruction(instruction);
        assert_eq!(0xB2, chip8.registers[0x05]);
    }

    #[test]
    fn opcode_8xy2_test() {
        let instruction = 0x8E02;
        let mut chip8 = Chip8::new();
        chip8.registers[0x0E] = 0x78;
        chip8.registers[0x00] = 0x1F;

        chip8.handle_instruction(instruction);
        assert_eq!(0x18, chip8.registers[0x0E]);
    }

    #[test]
    fn opcode_8xy3_test() {
        let instruction = 0x80B3;
        let mut chip8 = Chip8::new();
        chip8.registers[0x00] = 0x9E;
        chip8.registers[0x0B] = 0x54;

        chip8.handle_instruction(instruction);
        assert_eq!(0xCA, chip8.registers[0x00]);
    }

    #[test]
    fn opcode_8xy4_no_overflow_test() {
        let instruction = 0x85C4;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 90;
        chip8.registers[0x0C] = 56;

        chip8.handle_instruction(instruction);
        assert_eq!(146, chip8.registers[0x05]);
        assert_eq!(0, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xy4_overflow_test() {
        let instruction = 0x85C4;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 160;
        chip8.registers[0x0C] = 160;

        chip8.handle_instruction(instruction);
        assert_eq!(64, chip8.registers[0x05]);
        assert_eq!(1, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xy5_no_underflow_test() {
        let instruction = 0x85C5;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 56;
        chip8.registers[0x0C] = 19;

        chip8.handle_instruction(instruction);
        assert_eq!(37, chip8.registers[0x05]);
        assert_eq!(1, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xy5_underflow_test() {
        let instruction = 0x85C5;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 56;
        chip8.registers[0x0C] = 70;

        chip8.handle_instruction(instruction);
        assert_eq!(242, chip8.registers[0x05]);
        assert_eq!(0, chip8.registers[0x0F]);
    }
    #[test]
    fn opcode_8xy6_test() {
        let instruction = 0x85C6;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 0b0110_0110;

        chip8.handle_instruction(instruction);
        assert_eq!(51, chip8.registers[0x05]);
        assert_eq!(0, chip8.registers[0x0F]);

        chip8.registers[0x05] = 0b1101_0111;
        chip8.handle_instruction(instruction);
        assert_eq!(107, chip8.registers[0x05]);
        assert_eq!(1, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xy7_no_underflow_test() {
        let instruction = 0x85C7;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 56;
        chip8.registers[0x0C] = 100;

        chip8.handle_instruction(instruction);
        assert_eq!(44, chip8.registers[0x05]);
        assert_eq!(1, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xy7_underflow_test() {
        let instruction = 0x85C7;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 60;
        chip8.registers[0x0C] = 5;

        chip8.handle_instruction(instruction);
        assert_eq!(201, chip8.registers[0x05]);
        assert_eq!(0, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_8xye_test() {
        let instruction = 0x85CE;
        let mut chip8 = Chip8::new();
        chip8.registers[0x05] = 0b1101_0110;

        chip8.handle_instruction(instruction);
        assert_eq!(172, chip8.registers[0x05]);
        assert_eq!(1, chip8.registers[0x0F]);

        chip8.registers[0x05] = 0b0101_0101;
        chip8.handle_instruction(instruction);
        assert_eq!(170, chip8.registers[0x05]);
        assert_eq!(0, chip8.registers[0x0F]);
    }

    #[test]
    fn opcode_annn_test() {
        let instruction = 0xA232;
        let mut chip8 = Chip8::new();

        chip8.handle_instruction(instruction);
        assert_eq!(0x232, chip8.instruction_pointer);
    }

    #[test]
    fn opcode_bnnn_test() {
        let instruction = 0xBA09;
        let mut chip8 = Chip8::new();

        chip8.registers[0x00] = 0x45;
        chip8.handle_instruction(instruction);
        assert_eq!(0xA4E, chip8.program_counter);
    }

    #[test]
    fn opcode_fx07_test() {
        let instruction = 0xF407;
        let mut chip8 = Chip8::new();

        chip8.delay_timer = 0x70;
        chip8.handle_instruction(instruction);
        assert_eq!(0x70, chip8.registers[0x04]);
    }

    #[test]
    fn opcode_fx15_test() {
        let instruction = 0x0F915;
        let mut chip8 = Chip8::new();

        chip8.registers[0x09] = 0x78;
        chip8.handle_instruction(instruction);
        assert_eq!(0x78, chip8.delay_timer);
    }
    #[test]
    fn opcode_fx1e_test() {
        let instruction = 0xFA1E;
        let mut chip8 = Chip8::new();

        chip8.instruction_pointer = 0x30;
        chip8.registers[0x0A] = 0x50;

        chip8.handle_instruction(instruction);
        assert_eq!(0x80, chip8.instruction_pointer);
    }

    #[test]
    fn opcode_fx33_test() {
        let instruction = 0xFC33;
        let mut chip8 = Chip8::new();
        chip8.instruction_pointer = 0x0350;
        chip8.registers[0x0C] = 129;

        let instruction_pointer_val = chip8.instruction_pointer as usize;
        chip8.handle_instruction(instruction);
        assert_eq!(1, chip8.memory[instruction_pointer_val]);
        assert_eq!(2, chip8.memory[instruction_pointer_val + 1]);
        assert_eq!(9, chip8.memory[instruction_pointer_val + 2]);

        chip8.instruction_pointer = 0x0450;
        chip8.registers[0x0C] = 65;

        let instruction_pointer_val = chip8.instruction_pointer as usize;
        chip8.handle_instruction(instruction);
        assert_eq!(0, chip8.memory[instruction_pointer_val]);
        assert_eq!(6, chip8.memory[instruction_pointer_val + 1]);
        assert_eq!(5, chip8.memory[instruction_pointer_val + 2]);
    }

    #[test]
    fn opcode_fx55_test() {
        let instruction = 0xF455;
        let mut chip8 = Chip8::new();

        chip8.registers[0x00] = 31;
        chip8.registers[0x01] = 243;
        chip8.registers[0x02] = 56;
        chip8.registers[0x03] = 0;
        chip8.registers[0x04] = 167;

        chip8.instruction_pointer = 0x923;
        chip8.handle_instruction(instruction);
        let ip = chip8.instruction_pointer as usize;

        assert_eq!(31, chip8.memory[ip]);
        assert_eq!(243, chip8.memory[ip + 1]);
        assert_eq!(56, chip8.memory[ip + 2]);
        assert_eq!(0, chip8.memory[ip + 3]);
        assert_eq!(167, chip8.memory[ip + 4]);
        assert_eq!(0, chip8.memory[ip + 5]);
    }

    #[test]
    fn opcode_fx65_test() {
        let instruction = 0xF565;
        let mut chip8 = Chip8::new();
        chip8.instruction_pointer = 0x543;
        let ip = chip8.instruction_pointer as usize;

        chip8.memory[ip] = 0x1E;
        chip8.memory[ip + 1] = 0x45;
        chip8.memory[ip + 2] = 0xAA;
        chip8.memory[ip + 3] = 0x7E;
        chip8.memory[ip + 4] = 0xEF;

        chip8.handle_instruction(instruction);
        assert_eq!(0x1E, chip8.registers[0x00]);
        assert_eq!(0x45, chip8.registers[0x01]);
        assert_eq!(0xAA, chip8.registers[0x02]);
        assert_eq!(0x7E, chip8.registers[0x03]);
        assert_eq!(0xEF, chip8.registers[0x04]);
        assert_eq!(0x00, chip8.registers[0x05]);
    }
}
