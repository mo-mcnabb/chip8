extern crate sdl2;
use crate::renderer::Renderer;
use sdl2::EventPump;

pub struct DisplayDriver {
    pub renderer: Renderer,
    pub event_pump: EventPump,
}

impl DisplayDriver {
    pub fn build() -> Result<DisplayDriver, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", 640, 320)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl_context.event_pump()?;
        let renderer = Renderer::new(window)?;
        Ok(DisplayDriver {
            renderer: renderer,
            event_pump: event_pump,
        })
    }

    pub fn get_window_dimensions(&self) -> (u32, u32) {
        self.renderer.get_size().unwrap()
    }
}
