extern crate sdl2;
use crate::pixel::Pixel;
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

    fn flip_bit(&mut self, pixel: &mut Pixel) {
        let color = if pixel.on { Color::WHITE } else { Color::BLACK };

        self.canvas.set_draw_color(color);
        pixel.on = !pixel.on;

        self.canvas.fill_rect(Rect::new(
            (pixel.x * pixel.x_scale) as i32,
            (pixel.y * pixel.y_scale) as i32,
            pixel.x_scale,
            pixel.y_scale,
        ));

        println!("x_scale: {}", pixel.x_scale);
        println!("y_scale: {}", pixel.y_scale);
        println!("x location: {}", pixel.x);
        println!("y location: {} ", pixel.y);
    }

    pub fn draw(&mut self, pixels: &mut Vec<Vec<Pixel>>) {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        pixels
            .iter_mut()
            .for_each(|pixel_row| pixel_row.iter_mut().for_each(|pixel| self.flip_bit(pixel)));

        self.canvas.present();
    }
}
