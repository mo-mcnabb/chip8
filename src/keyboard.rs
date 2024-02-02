use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

pub struct Keyboard {
    state: [bool; 16],
    halt_key: usize,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            state: [false; 16],
            halt_key: 256, //256 bc i want to embarrass bytes and their storage capacity
        }
    }

    pub fn parse_keyboard_event(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return false,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = Keyboard::get_key_pressed(keycode) {
                        self.state[key] = true;
                    }
                }
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => {
                    if let Some(key) = Keyboard::get_key_pressed(keycode) {
                        self.state[key] = false;
                    }
                }
                _ => {}
            }
        }

        true
    }

    pub fn get_state(&self) -> &[bool; 16] {
        &self.state
    }

    pub fn set_halt_key(&mut self, key: u8) {
        self.halt_key = key as usize;
    }

    pub fn get_halt_key(&self) -> usize {
        self.halt_key as usize
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
}
