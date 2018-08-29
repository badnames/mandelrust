extern crate num_complex;
extern crate palette;

use render::num_complex::Complex64;
use self::palette::{Srgb, LinSrgb, Lch, Pixel, Hue};

pub fn render_mandelbrot(width: u32, height: u32, x_pos: f64, y_pos: f64, scale: f64, max_itterations: u32) -> Vec<u8> {

    let array_size = (3 * width * height) as usize;
    
    let mut pixels: Vec<u8> = vec![0xFF; array_size];

    println!("rendering...");

    for y in 0..height { 
        for x in 0..width {
            let transformed = Complex64::new( (x as f64) / ((width) as f64)  * scale - scale / 2.0 + x_pos,
                                              (y as f64) / ((height) as f64) * scale - scale / 2.0 + y_pos);

            let itterations = itterate(transformed, max_itterations);

            if itterations == max_itterations {
                pixels[coordinates_to_array_index(width, x, y) + 0] = 0x00; //RED
                pixels[coordinates_to_array_index(width, x, y) + 1] = 0x00; //GREEN
                pixels[coordinates_to_array_index(width, x, y) + 2] = 0x00; //BLUE
                continue;
            }

            let base_color: Lch = Srgb::new(0.8, 0.2, 0.1).into();

            let color = LinSrgb::from(
                base_color.shift_hue(
                    interpolate_hue(itterations as f32, 0.0, max_itterations as f32)
                )
            );

            let color_raw: [u8; 3] = Srgb::from_linear(color.into()).into_format().into_raw();

            pixels[coordinates_to_array_index(width, x, y) + 0] = color_raw[0];
            pixels[coordinates_to_array_index(width, x, y) + 1] = color_raw[1];
            pixels[coordinates_to_array_index(width, x, y) + 2] = color_raw[2];
        }
    }

    println!("done");

    pixels
}

//formula from https://en.wikipedia.org/wiki/Mandelbrot_set#Formal_definition
fn itterate(c: Complex64, max_itterations: u32) -> u32 {
    let mut z = Complex64::new(0.0, 0.0);

    for itteration in 1..max_itterations {
        z = z.powf(2.0) + c;

        if (z.re * z.re + z.im * z.im) > 4.0 {
            return itteration;
        }  
    }

    max_itterations

}

fn interpolate_hue(value: f32, min: f32, max: f32) -> f32 {
    if value < min || value > max {
        panic!("value has to be between min and max");
    }

    ((value - min) / (max - min)) * 360.0
}

fn coordinates_to_array_index(width: u32, x: u32, y: u32) -> usize {
    ((y * width * 3) + (x * 3)) as usize
}
