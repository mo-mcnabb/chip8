extern crate sdl2;

const DEFAULT_CHIP8_PIXEL_HEIGHT: u32 = 64;
const DEFAULT_CHIP8_PIXEL_WIDTH: u32 = 32;

#[derive(Debug)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub on: bool,
    pub x_scale: u32,
    pub y_scale: u32,
}

impl Pixel {
    pub fn new(x: u32, y: u32, on: bool, x_scale: u32, y_scale: u32) -> Pixel {
        Pixel {
            x: x,
            y: y,
            on: on,
            x_scale: x_scale,
            y_scale: y_scale,
        }
    }

    pub fn build_from_window_size(height: u32, width: u32) -> Result<Vec<Vec<Pixel>>, String> {
        println!("height: {height}");
        println!("width: {width}");
        if height % DEFAULT_CHIP8_PIXEL_HEIGHT != 0 {
            println!("HEIHGT IS FUCKED UO SOMEHOW");
            return Err(format!("Window height is not evenly divisible by default height. Window height: {}, default height: {}", height, DEFAULT_CHIP8_PIXEL_HEIGHT));
        }

        if width % DEFAULT_CHIP8_PIXEL_WIDTH != 0 {
            println!("WIDTH IS FUCKED???");
            return Err(format!("Window width is not evenly divisible by default width. Window width: {}, default width: {}", width, DEFAULT_CHIP8_PIXEL_WIDTH));
        }

        let height_scale = height / DEFAULT_CHIP8_PIXEL_HEIGHT;
        let width_scale = width / DEFAULT_CHIP8_PIXEL_WIDTH;

        let mut pixels: Vec<Vec<Pixel>> = Vec::new();
        // 64 x 32 pixels
        for y_location in 0..DEFAULT_CHIP8_PIXEL_WIDTH {
            let mut height_vec: Vec<Pixel> = Vec::new();
            for x_location in 0..DEFAULT_CHIP8_PIXEL_HEIGHT {
                let on = x_location % 2 == 0;
                height_vec.push(Pixel::new(
                    x_location,
                    y_location,
                    on,
                    height_scale,
                    width_scale,
                ));
            }
            pixels.push(height_vec);
        }

        Ok(pixels)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn individual_pixel_initialization_test() {
        let pixel = Pixel::new(10, 20, true, 1, 1);

        assert_eq!(10, pixel.x);
        assert_eq!(20, pixel.y);
        assert_eq!(true, pixel.on);
        assert_eq!(1, pixel.x_scale);
        assert_eq!(1, pixel.y_scale);
    }

    #[test]
    fn bulk_pixel_initialization_test() {
        let pixels = Pixel::build_from_window_size(640, 320).unwrap();

        assert_eq!(64, pixels.len());
        assert_eq!(32, pixels[0].len());
        assert_eq!(10, pixels[0][0].x_scale);
        assert_eq!(10, pixels[0][0].y_scale);
        assert_eq!(true, pixels[0][0].on);
        assert_eq!(false, pixels[0][1].on);
    }

    #[test]
    fn buld_pixel_width_fail_test() {
        let pixels = Pixel::build_from_window_size(640, 300);
        assert!(pixels.is_err());
        assert_eq!(
        pixels.unwrap_err(),
        "Window width is not evenly divisible by default width. Window width: 300, default width: 32"
    );
    }
}
