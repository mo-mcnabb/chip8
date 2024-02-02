use crate::audio::AudioDriver;
use crate::chip8::Chip8;
use crate::display::DisplayDriver;
use crate::keyboard::Keyboard;
use std::time::{Duration, Instant};

pub struct Emulator {
    audio_driver: AudioDriver,
    display_driver: DisplayDriver,
    chip8_processor: Chip8,
    keyboard: Keyboard,
}

impl Emulator {
    pub fn build() -> Result<Emulator, String> {
        let audio_driver = AudioDriver::build();
        let display_driver = DisplayDriver::build()?;
        let mut chip8_processor = Chip8::new();
        let (width, height) = display_driver.get_window_dimensions();
        chip8_processor.initialize_pixels(height, width)?;
        let keyboard = Keyboard::new();

        Ok(Emulator {
            audio_driver,
            display_driver,
            chip8_processor,
            keyboard,
        })
    }

    pub fn run(&mut self, rom: String) -> Result<(), String> {
        self.chip8_processor.load_rom(rom)?;

        let interval = Duration::from_nanos(1_000_000_000u64 / 60);
        let mut last_tick = Instant::now();

        'running: loop {
            if !self
                .keyboard
                .parse_keyboard_event(&mut self.display_driver.event_pump)
            {
                break 'running;
            }

            self.chip8_processor
                .handle_next_instruction(&mut self.keyboard, &mut self.display_driver.event_pump);
            if self.chip8_processor.vram_changed {
                self.display_driver.renderer.draw(&mut self.chip8_processor);
            }
            let now = Instant::now();
            if now.duration_since(last_tick) >= interval {
                self.chip8_processor.decrement_sound_timer();
                self.chip8_processor.decrement_delay_timer();
                last_tick = now;
            }

            if self.chip8_processor.get_sound_timer() > 0 {
                self.audio_driver.play();
            } else {
                self.audio_driver.stop();
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1000));
        }

        Ok(())
    }
}
