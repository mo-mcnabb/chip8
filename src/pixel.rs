extern crate sdl2;

#[derive(Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub on: bool,
}

impl Pixel {
    pub fn new(x: u32, y: u32, on: bool) -> Pixel {
        Pixel { x: x, y: y, on: on }
    }

    pub fn set(&mut self, on: bool) {
        self.on = on;
    }

    pub fn turn_off(&mut self) {
        self.on = false;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn individual_pixel_initialization_test() {
        let pixel = Pixel::new(10, 20, true);

        assert_eq!(10, pixel.x);
        assert_eq!(20, pixel.y);
        assert_eq!(true, pixel.on);
        //assert_eq!(1, pixel.x_scale);
        //assert_eq!(1, pixel.y_scale);
    }
}
