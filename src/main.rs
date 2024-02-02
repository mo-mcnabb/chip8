extern crate sdl2;
mod audio;
mod chip8;
mod display;
mod emulator;
mod keyboard;
mod pixel;
mod renderer;
use crate::emulator::Emulator;

pub fn main() -> Result<(), String> {
    let mut emulator = Emulator::build()?;
    emulator.run(String::from("roms/particle_demo.ch8"))?;

    Ok(())
}
