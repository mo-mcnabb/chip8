extern crate rand;
use crate::keyboard::Keyboard;
use crate::pixel::Pixel;
use rand::prelude::Rng;
use sdl2::EventPump;
use std::fs;

const DEFAULT_CHIP8_PIXEL_HEIGHT: u32 = 32;
const DEFAULT_CHIP8_PIXEL_WIDTH: u32 = 64;

pub struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    index_register: u16,
    stack: Vec<u16>,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
    pub vram: Vec<Vec<Pixel>>,
    pub vram_changed: bool,
    pub vram_scale: usize,
    awaiting_keypress: bool,
    awaiting_keylift: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            memory: [0; 4096],
            registers: [0; 16],
            index_register: 0,
            stack: Vec::new(),
            program_counter: 0x200, // programs start at 0x200
            delay_timer: 0,
            sound_timer: 0,
            vram: Vec::new(),
            vram_changed: false,
            vram_scale: 1,
            awaiting_keypress: false,
            awaiting_keylift: false,
        };

        chip8.load_sprites_into_memory();
        chip8
    }

    pub fn load_sprites_into_memory(&mut self) {
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

    pub fn load_rom(&mut self, rom_file_path: String) -> Result<(), String> {
        let rom_contents = fs::read(rom_file_path).expect("Error opening file");

        let mem_start_location = 0x200;
        for index in 0..rom_contents.len() {
            self.memory[mem_start_location + index] = rom_contents[index];
        }

        Ok(())
    }

    pub fn initialize_pixels(&mut self, height: u32, width: u32) -> Result<(), String> {
        println!("H: {}", height);
        println!("W: {}", width);
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
        for y_location in 0..DEFAULT_CHIP8_PIXEL_HEIGHT {
            let mut row: Vec<Pixel> = Vec::new();
            for x_location in 0..DEFAULT_CHIP8_PIXEL_WIDTH {
                row.push(Pixel::new(x_location, y_location, false));
            }
            self.vram.push(row);
        }

        Ok(())
    }

    pub fn set_register_value(&mut self, register: u8, value: u8) {
        self.registers[register as usize] = value;
    }

    pub fn decrement_sound_timer(&mut self) {
        if self.sound_timer != 0 {
            self.sound_timer = self.sound_timer - 1;
        }
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }

    pub fn play_sound(&self) {
        //play sound
        //println!("Playing sound");
    }

    pub fn decrement_delay_timer(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer = self.delay_timer - 1;
        }
    }

    fn decode(left_byte: u8, right_byte: u8) -> u16 {
        let left_byte = left_byte as u16;
        let right_byte = right_byte as u16;

        (left_byte << 8) | right_byte
    }

    pub fn handle_next_instruction(&mut self, keyboard: &mut Keyboard, event_pump: &mut EventPump) {
        let instruction = Chip8::decode(
            self.memory[self.program_counter as usize],
            self.memory[self.program_counter as usize + 1],
        );
        //println!("{:#x}", instruction);
        let x_index = ((instruction & 0x0F00) >> 8) as usize;
        let y_index = ((instruction & 0x00F0) >> 4) as usize;
        let nnn = instruction & 0x0FFF; // for instructions like ANNN, BNNN, etc.
        let nn = (instruction & 0x00FF) as u8; // for instruction like 6XNN, 7XNN, etc.
        let n = (instruction & 0x000F) as u8; // for instructions like DXYN
        let mut increment_program_counter = true;

        match instruction >> 12 {
            0x0 => match instruction & 0x00FF {
                0x00E0 => {
                    // ("clear diplay");
                    self.vram.iter_mut().for_each(|row| {
                        row.iter_mut().for_each(|pixel| {
                            pixel.turn_off();
                        });
                    });
                    self.vram_changed = true;
                }
                0x00EE => {
                    //  ("return");
                    self.program_counter = self.stack.pop().unwrap();
                }
                _ => {}
            },
            0x1 => {
                // ("goto NNN");
                self.program_counter = nnn;
                increment_program_counter = false;
            }
            0x2 => {
                // ("call subroutine at NNN");
                self.stack.push(self.program_counter);
                self.program_counter = nnn;
                increment_program_counter = false;
            }
            0x3 => {
                // ("conditional, 3XNN: skips next instruction if Vx = NN");
                if self.registers[x_index] == nn {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0x4 => {
                // ("conditional, 4XNN: skips next instruction if Vx != NN");
                if self.registers[x_index] != nn {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0x5 => {
                // ("conditional, 5XY0: skips next instruction if Vx == Vy");
                if self.registers[x_index] == self.registers[y_index] {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0x6 => {
                // ("6XNN: sets Vx to NN");
                self.registers[x_index] = nn;
            }
            0x7 => {
                // ("7XNN: adds NN to Vx (carry flag not changed)");
                self.registers[x_index] = self.registers[x_index].wrapping_add(nn);
            }
            0x8 => match instruction & 0x000F {
                0x0 => {
                    // ("8XY0: sets Vx to Vy");
                    self.registers[x_index] = self.registers[y_index];
                }
                0x1 => {
                    // ("8XY1: sets Vx to Vx | Vy. Vx = Vx | Vy");
                    self.registers[x_index] = self.registers[x_index] | self.registers[y_index];
                }
                0x2 => {
                    // ("8XY2: sets Vx to Vx & Vy. Vx = Vx & Vy");
                    self.registers[x_index] = self.registers[x_index] & self.registers[y_index];
                }
                0x3 => {
                    // ("8XY3: sets Vx to Vx xor Vy. Vx = Vx ^ Vy");
                    self.registers[x_index] = self.registers[x_index] ^ self.registers[y_index];
                }
                0x4 => {
                    // ("8XY4: Adds Vy to Vx. VF(carry flag) is set to 1 when there's an overflow, and to 0 when there is not");
                    match self.registers[x_index].checked_add(self.registers[y_index]) {
                        Some(output) => {
                            self.registers[x_index] = output;
                            self.registers[0x0F] = 0;
                        }
                        None => {
                            self.registers[x_index] =
                                self.registers[x_index].wrapping_add(self.registers[y_index]);
                            self.registers[0x0F] = 1;
                        }
                    }
                }
                0x5 => {
                    // ("8XY5: Vy is subtracted from Vx. VF (carry flag) is set to 0 when there is an underflow, and 1 when there is not. (VF = 1 if Vx >= Vy and 0 if not)");
                    match self.registers[x_index].checked_sub(self.registers[y_index]) {
                        Some(output) => {
                            self.registers[x_index] = output;
                            self.registers[0x0F] = 1;
                        }
                        None => {
                            self.registers[x_index] =
                                self.registers[x_index].wrapping_sub(self.registers[y_index]);
                            self.registers[0x0F] = 0;
                        }
                    }
                }
                0x6 => {
                    //                     ("8XY6: stores to least significant bit of Vx in VF and then shifts Vx to the right by 1. Vx = Vx >> 1");

                    let least_sig_bit = self.registers[x_index] & 0x01;
                    self.registers[x_index] = self.registers[x_index] >> 1;
                    self.registers[0x0F] = least_sig_bit;
                }
                0x7 => {
                    // ("8XY7: sets Vx to Vy minus Vx. Vf is set to 0 when there is an underflow, and 1 when there is not. i.e. VF = 1 when Vy >= Vx");
                    match self.registers[y_index].checked_sub(self.registers[x_index]) {
                        Some(output) => {
                            self.registers[x_index] = output;
                            self.registers[0x0F] = 1;
                        }
                        None => {
                            self.registers[x_index] =
                                self.registers[y_index].wrapping_sub(self.registers[x_index]);
                            self.registers[0x0F] = 0;
                        }
                    }
                }
                0xE => {
                    // ("8XYE: stores the most significant bit in VF and shifts VX to the left by 1. Vx = Vx << 1");
                    let most_sig_bit = (self.registers[x_index] & 0b1000_0000) >> 7;
                    self.registers[x_index] = self.registers[x_index] << 1;
                    self.registers[0x0F] = most_sig_bit;
                }
                _ => println!("invalide opcode"),
            },
            0x9 => {
                // ("9XY0: skips the next instruction if Vx != Vy");
                if self.registers[x_index] != self.registers[y_index] {
                    self.program_counter = self.program_counter + 2;
                }
            }
            0xA => {
                // ("ANNN: Sets the I(instruction) address to NNN");
                self.index_register = nnn;
                //increment_program_counter = false;
            }
            0xB => {
                // ("BNNN: jumps to the address NNN plus V0. PC(program counter) = V0 + NNN");
                self.program_counter = self.registers[0x00] as u16 + nnn;
            }
            0xC => {
                // ("CXNN: sets Vx to the result of a bitwise and operation on a random number (typically 0 to 255) and NN. Vx = rand() & NN");
                let random_number: u8 = rand::thread_rng().gen();
                self.registers[x_index] = random_number & nn;
            }
            0xD => {
                // ("DXYN: Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. Each row of 8 pixels is read as bit-coded starting from memory location I; I value does not change after the execution of this instruction. As described above, VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen");
                //let ip = self.program_counter as usize;
                self.registers[0x0F] = 0;
                let x_location = self.registers[x_index];
                let y_location = self.registers[y_index];
                let index = self.index_register as usize;

                for row_offset in 0..n as u32 {
                    let row_byte = self.memory[index + row_offset as usize];
                    for column_offset in 0..8 {
                        let bit_shift_amount = 7 - column_offset;
                        let and_val = 0b1000_0000 >> column_offset;
                        let pixel_val = ((row_byte & and_val) >> bit_shift_amount) == 1;

                        let y_wrapped = ((y_location as u32 + row_offset)
                            % DEFAULT_CHIP8_PIXEL_HEIGHT)
                            as usize;
                        let x_wrapped = ((x_location as u32 + column_offset)
                            % DEFAULT_CHIP8_PIXEL_WIDTH)
                            as usize;
                        if pixel_val && self.vram[y_wrapped][x_wrapped].on {
                            self.registers[0x0F] = 1;
                            self.vram[y_wrapped][x_wrapped].turn_off();
                        } else if pixel_val && !self.vram[y_wrapped][x_wrapped].on {
                            self.vram[y_wrapped][x_wrapped].set(pixel_val);
                        }
                    }
                }

                self.vram_changed = true;
            }
            0xE => match instruction & 0x00F0 {
                0x0090 => {
                    // ("EX9E: skips the next instruction if the key stored in Vx is pressed (usually the next instruction is a jump to skip a code block). if(key() == Vx)");
                    let key = self.registers[x_index];
                    if keyboard.get_state()[key as usize] {
                        self.program_counter = self.program_counter + 2;
                    }
                }
                0x00A0 => {
                    // ("EXA1: skips the next instruction if the key stored in Vx is not pressed (usually the next instruction is a jump to skip a code block. if (key() != Vx))");
                    let key = self.registers[x_index];
                    if !keyboard.get_state()[key as usize] {
                        self.program_counter = self.program_counter + 2;
                    }
                }
                _ => {
                    ("invalid opcode");
                }
            },
            0xF => match instruction & 0x00FF {
                0x0007 => {
                    // ("FX07: sets vx to the value of the delay timer. Vx = get_delay()");
                    self.registers[x_index] = self.delay_timer;
                }
                0x000A => {
                    // ("FX0A: A key press is awaited, and then stored in Vx (blocking operation, all instruction halted until next key event. probably a loop?)");
                    self.awaiting_keypress = true;
                    if self.awaiting_keypress {
                        if let Some(key) = keyboard.get_state().iter().position(|&state| state) {
                            keyboard.set_halt_key(key as u8);
                            self.awaiting_keylift = true;
                            self.awaiting_keypress = false;
                        }
                    }

                    if self.awaiting_keylift {
                        if !keyboard.get_state()[keyboard.get_halt_key()] {
                            self.awaiting_keylift = false;
                            self.registers[x_index] = keyboard.get_halt_key() as u8;
                        }
                    }
                    self.awaiting_keypress =
                        self.registers[x_index] != keyboard.get_halt_key() as u8;

                    if self.awaiting_keypress || self.awaiting_keylift {
                        self.program_counter = self.program_counter - 2;
                    }
                }
                0x0015 => {
                    // ("FX15: sets the delay timer to Vx. delay_timer(Vx)");
                    self.delay_timer = self.registers[x_index];
                }
                0x0018 => {
                    // ("FX18: sets the sound timer to Vx. sound_timer(Vx);")
                    self.sound_timer = self.registers[x_index];
                }
                0x001E => {
                    // ("FX1E: Adds Vx to I. VF is not affected. I = I + Vx");
                    self.index_register = self.index_register + self.registers[x_index] as u16;
                }
                0x0029 => {
                    // ("FX29: sets I to the location of the sprite for the character in Vx. characters 0-F in hex are represented by a 4x5 font. I = sprite_addr[Vx]");
                    let sprite_addr = self.registers[x_index] * 5;
                    self.index_register = sprite_addr as u16;
                }
                0x0033 => {
                    // ("FX33: stores the binary-codeddecimal representation of Vx, with the hundreds digit in memory at location I, the tens digit at location I+1, and the ones digit at locaion I + 2");
                    let register_x_val = self.registers[x_index];
                    let hundreds = (register_x_val / 100) % 10;
                    let tens = (register_x_val / 10) % 10;
                    let ones = register_x_val % 10;
                    let index = self.index_register as usize;

                    self.memory[index] = hundreds;
                    self.memory[index + 1] = tens;
                    self.memory[index + 2] = ones;
                }
                0x0055 => {
                    // ("FX55: stores from V0 to Vx (including Vx) in memory, starting at address I. the offset from I is increased by 1 for each value written, but I itself is left unmodified. reg_dum(Vx, &I)");
                    let i = self.index_register as usize;
                    self.registers
                        .iter()
                        .take(x_index + 1) //+1 bc zero index
                        .enumerate()
                        .for_each(|(index, register)| {
                            self.memory[i + index] = *register;
                        });
                }
                0x0065 => {
                    // ("FX65: Fills from V0 to Vx (including Vx) with values from memory, starting at address I. the offset from I is increased by 1 for each value read, but I remains umodified.");
                    let i = self.index_register as usize;
                    let mem_slice = &self.memory[i..i + x_index + 1];

                    mem_slice.iter().enumerate().for_each(|(index, mem_val)| {
                        self.registers[index] = *mem_val;
                    });
                }
                _ => {}
            },
            _ => (),
        };

        if increment_program_counter {
            self.program_counter = self.program_counter + 2;
        }
    }
}
