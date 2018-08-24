extern crate sdl2;
extern crate gl;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::PixelFormatEnum;



const WIDTH: u32 = 400;
const HEIGHT: u32 = 400; 

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

    let textureCreator = canvas.texture_creator();

    let mut pixels = render_mandelbrot();
    
    let surface = match Surface::from_data(&mut pixels, WIDTH, HEIGHT, 3 * WIDTH, PixelFormatEnum::RGB24) {
        Ok(surface) => surface,
        Err(err) => panic!("Invalid surface generated: {}", err)
    };
    
    let texture = textureCreator.create_texture_from_surface(surface).unwrap();

    gl::load_with(|name| context_video.gl_get_proc_address(name) as *const _);
    canvas.window().gl_set_context_to_current();
    
    unsafe {
        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }

    canvas.copy(&texture, None, None);

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

fn render_mandelbrot<'a>() -> [u8; (3 * WIDTH * HEIGHT) as usize] {

    const ARRAY_SIZE: usize = (3 * WIDTH * HEIGHT) as usize;
    let pixels: [u8; ARRAY_SIZE] = [0xFF; ARRAY_SIZE];

    return pixels;
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}