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

    let mut event_pump = sdl_context.event_pump()?;
    let mut keyboard_state = [false; 16];
    chip8.load_rom(String::from("roms/IBM Logo.ch8"))?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = get_key_pressed(keycode) {
                        keyboard_state[key] = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = get_key_pressed(keycode) {
                        keyboard_state[key] = false;
                    }
                }
                _ => {}
            }
        }

        chip8.handle_next_instruction(&keyboard_state);
        if chip8.vram_changed {
            renderer.draw(&mut chip8);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
        // The rest of the game loop goes here...
    }
    Ok(())
}

fn get_key_pressed(key_pressed: Keycode) -> Option<usize> {
    match key_pressed {
        Keycode::Num0 => Some(0),
        Keycode::Num1 => Some(1),
        Keycode::Num2 => Some(2),
        Keycode::Num3 => Some(3),
        Keycode::Num4 => Some(4),
        Keycode::Num5 => Some(5),
        Keycode::Num6 => Some(6),
        Keycode::Num7 => Some(7),
        Keycode::Num8 => Some(8),
        Keycode::Num9 => Some(9),
        Keycode::A => Some(10),
        Keycode::B => Some(11),
        Keycode::C => Some(12),
        Keycode::D => Some(13),
        Keycode::E => Some(14),
        Keycode::F => Some(15),
        _ => None,
    }
}
