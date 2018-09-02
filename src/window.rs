extern crate sdl2;
extern crate gl;

use window::sdl2::event::{Event};
use window::sdl2::surface::{Surface};
use window::sdl2::pixels::PixelFormatEnum;

use std::sync::{Arc, Mutex};

use render;

pub fn start(render_args_ref: Arc<render::RenderArgs>) {

    let mut render_args = Arc::clone(&render_args_ref);

    //setup SDL
    let context = sdl2::init().unwrap();
    let context_video = context.video().unwrap();
    
    let window = context_video.window("Mandelbrot Renderer", render_args.width, render_args.height)
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
    
    let buffer_ref = Arc::new(
        Mutex::new(
            vec![0x00; (render_args.width * render_args.height * 3) as usize]
        )
    );

    render::render_mandelbrot(&render_args_ref, &buffer_ref);
    
    let buffer_ref = Arc::clone(&buffer_ref);
    let mut buffer = buffer_ref.lock().unwrap();

    let surface = match Surface::from_data(&mut buffer[..], render_args.width, render_args.height, 3 * render_args.width, PixelFormatEnum::RGB24) {
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