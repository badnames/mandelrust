extern crate sdl2;
extern crate gl;

mod render;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::PixelFormatEnum;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600; 

const X_POS: f64 = -0.0091275;
const Y_POS: f64= 0.7899912;

const MAX_ITTERATIONS: u32 = 200;

const SCALE: f64 = 0.1;

fn main() {

    //setup SDL
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
    


    let mut pixels = render::render_mandelbrot(WIDTH, HEIGHT, X_POS, Y_POS, SCALE, MAX_ITTERATIONS);
    


    let surface = match Surface::from_data(&mut pixels[..], WIDTH, HEIGHT, 3 * WIDTH, PixelFormatEnum::RGB24) {
        Ok(surface) => surface,
        Err(err) => panic!("Invalid surface generated: {}", err)
    };
    
    let texture = texture_creator.create_texture_from_surface(surface).unwrap();
    let _ = canvas.copy(&texture, None, None);

    canvas.present();

    let mut events = context.event_pump().unwrap();    

    'main_loop : loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit{..} => break 'main_loop,
                _               => continue
            }
        } 
    }

}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }

    None
}