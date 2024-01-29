extern crate sdl2;
mod chip8;
mod pixel;
mod renderer;
use crate::chip8::Chip8;
use crate::renderer::Renderer;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", 640, 320)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let (height, width) = &window.size();
    let mut renderer = Renderer::new(window)?;

    let mut chip8 = Chip8::new();
    chip8.initialize_pixels(*height, *width);
    chip8.set_register_value(0x03, 32);
    chip8.set_register_value(0x05, 16);

    let instruction: u16 = 0xD355;
    chip8.handle_instruction(instruction);
    //println!("{:#?}", pixels);
    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        if chip8.vram_changed {
            renderer.draw(&mut chip8);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }

    Ok(())
}
