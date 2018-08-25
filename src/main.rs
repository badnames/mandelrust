extern crate sdl2;
extern crate gl;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::PixelFormatEnum;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600; 

const X_POS: f64 = -0.0091275;
const Y_POS: f64= 0.7899912;

const MAX_ITTERATIONS: u8 = 200;

static mut SCALE: f64 = 0.1;

fn main() {
    let context = sdl2::init().unwrap();
    let context_video = context.video().unwrap();
    
    let window = context_video.window("Mandelbrot Renderer", WIDTH, HEIGHT)
    .position_centered()
    .opengl()
    .build()
    .unwrap();

    let mut canvas = window.into_canvas()
    .index(find_sdl_gl_driver().unwrap())
    .build()
    .unwrap();

    let texture_creator = canvas.texture_creator();

    gl::load_with(|name| context_video.gl_get_proc_address(name) as *const _);
    let _ = canvas.window().gl_set_context_to_current();
    
    let mut events = context.event_pump().unwrap();    

    'main_loop : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main_loop,
                _               => continue
            }
        } 

        let mut pixels = render_mandelbrot();
    
        let surface = match Surface::from_data(&mut pixels, WIDTH, HEIGHT, 3 * WIDTH, PixelFormatEnum::RGB24) {
            Ok(surface) => surface,
            Err(err) => panic!("Invalid surface generated: {}", err)
        };
        
        let texture = texture_creator.create_texture_from_surface(surface).unwrap();

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let _ = canvas.copy(&texture, None, None);

        canvas.present();

        unsafe {
            SCALE /= 1.001;
        }
    }

}

fn render_mandelbrot<'a>() -> [u8; (3 * WIDTH * HEIGHT) as usize] {

    const ARRAY_SIZE: usize = (3 * WIDTH * HEIGHT) as usize;
    let mut pixels: [u8; ARRAY_SIZE] = [0xFF; ARRAY_SIZE];

    println!("rendering...");

    for y in 0..HEIGHT {
        
        for x in 0..WIDTH {
            let mut x_scaled: f64;
            let mut y_scaled: f64;

            unsafe {
                x_scaled = (x as f64) / ((WIDTH) as f64)  * SCALE - SCALE / 2.0 + X_POS;
                y_scaled = (y as f64) / ((HEIGHT) as f64) * SCALE - SCALE / 2.0 + Y_POS;
            }

            let itterations = itterate(x_scaled, y_scaled, MAX_ITTERATIONS);

            //make every 10th row of pixels red
            if itterations == MAX_ITTERATIONS {
                pixels[coordinates_to_array_index(x, y) + 0] = 0x00; //RED
                pixels[coordinates_to_array_index(x, y) + 1] = 0x00; //GREEN
                pixels[coordinates_to_array_index(x, y) + 2] = 0x00; //BLUE
                continue;
            }
        }
    }

    println!("done");

    return pixels;
}

fn itterate(x: f64, y: f64, max_itterations: u8) -> u8 {
    let mut curr_x: f64 = 0.0;
    let mut curr_y: f64 = 0.0;

    for itteration in 1..max_itterations {
        let xx = curr_x * curr_x;
        let yy = curr_y * curr_y;
        let xy = curr_x * curr_y;

        curr_x = (xx - yy) + x;
        curr_y = 2.0 * xy + y;

        if (curr_x * curr_x + curr_y * curr_y) > 4.0 {
            return itteration;
        }  
    }

    max_itterations

}

fn coordinates_to_array_index(x: u32, y: u32) -> usize {
    ((y * WIDTH * 3) + (x * 3)) as usize
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }

    None
}