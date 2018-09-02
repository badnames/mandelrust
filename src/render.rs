extern crate num_complex;
extern crate palette;
extern crate num_cpus;

use std::sync::{Mutex, Arc};
use std::thread;

use render::num_complex::Complex64;
use self::palette::{Srgb, LinSrgb, Lch, Pixel, Hue};

#[derive(Copy, Clone)]
pub struct RenderArgs {
    pub width:           u32,
    pub height:          u32,
    pub x_pos:           f64,
    pub y_pos:           f64,
    pub scale:           f64,
    pub max_itterations: u32
}

pub fn render_mandelbrot(render_args_ref: &Arc<RenderArgs>, pixels:& Arc<Mutex<Vec<u8>>>) {

    let args = Arc::clone(render_args_ref);
     
    let mut handles = vec![];

    println!("rendering on {} cpus", num_cpus::get());


    for num_cpu in 0..num_cpus::get() {
        let pixelRef = Arc::clone(pixels);
        let render_args_ref  = Arc::clone(render_args_ref);

        let partition_height = args.height / num_cpus::get() as u32;

        let partition_start_x = 0;
        let partition_start_y = partition_height * num_cpu as u32;

        let partition_end_x = args.width;
        let partition_end_y = partition_height * (num_cpu as u32 + 1);
        
        let handle = thread::spawn(move || {
            println!("starting thread #{}", num_cpu);
            subRender(render_args_ref, pixelRef, partition_start_x, partition_end_x, partition_start_y, partition_end_y);
            println!("finishing thread #{}", num_cpu);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("done");

}

fn subRender(argsRef: Arc<RenderArgs>, pixelRef: Arc<Mutex<Vec<u8>>>, start_x: u32, end_x: u32, start_y: u32, end_y: u32) {

    let pixelRef   = Arc::clone(&pixelRef);
    let mut pixels = pixelRef.lock().unwrap();
    let     args   = Arc::clone(&argsRef);      

    for y in start_y..end_y { 
        for x in start_x..end_x {
            let transformed = Complex64::new( (x as f64) / ((args.width) as f64)  * args.scale - args.scale / 2.0 + args.x_pos,
                                            (y as f64) / ((args.width) as f64)  * args.scale - args.scale / 2.0 + args.y_pos);

            let itterations = itterate(transformed, args.max_itterations);

            if itterations == args.max_itterations {
                pixels[coordinates_to_array_index(args.width, x, y) + 0] = 0x00; //RED
                pixels[coordinates_to_array_index(args.width, x, y) + 1] = 0x00; //GREEN
                pixels[coordinates_to_array_index(args.width, x, y) + 2] = 0x00; //BLUE
                continue;
            }

            let base_color: Lch = Srgb::new(0.8, 0.2, 0.1).into();

            let color = LinSrgb::from(
                base_color.shift_hue(
                    interpolate_hue(itterations as f32, 0.0, args.max_itterations as f32)
                )
            );

            let color_raw: [u8; 3] = Srgb::from_linear(color.into()).into_format().into_raw();

            pixels[coordinates_to_array_index(args.width, x, y) + 0] = color_raw[0];
            pixels[coordinates_to_array_index(args.width, x, y) + 1] = color_raw[1];
            pixels[coordinates_to_array_index(args.width, x, y) + 2] = color_raw[2];
        }
    }
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
