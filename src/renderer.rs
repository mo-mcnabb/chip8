extern crate sdl2;
use crate::chip8::Chip8;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct Renderer {
    canvas: WindowCanvas,
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    pub fn get_size(&self) -> Result<(u32, u32), String> {
        self.canvas.output_size()
    }

    pub fn draw(&mut self, chip8: &mut Chip8) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        chip8.vram.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|pixel| {
                let color = if pixel.on { Color::WHITE } else { Color::BLACK };
                self.canvas.set_draw_color(color);

                self.canvas.fill_rect(Rect::new(
                    (pixel.x * chip8.vram_scale as u32) as i32,
                    (pixel.y * chip8.vram_scale as u32) as i32,
                    chip8.vram_scale as u32,
                    chip8.vram_scale as u32,
                ));
            });
        });

        self.canvas.present();
        chip8.vram_changed = false;
    }
}
