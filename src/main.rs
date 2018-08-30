extern crate sdl2;
extern crate gl;
extern crate argparse;

mod render;

use sdl2::event::{Event};
use sdl2::surface::{Surface};
use sdl2::pixels::PixelFormatEnum;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut console_args = render::RenderArgs {
        width: 800,
        height: 600,
        x_pos: 0.0,
        y_pos: 0.0,
        scale: 5.0,
        max_itterations: 50
    };

    //parse commandline arguments
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Renders a slice of the mandelbrot set.");
        
        parser.refer(&mut console_args.width)
            .add_option(&["-w", "--width"], Store,
            "the width of the window");
        parser.refer(&mut console_args.height)
            .add_option(&["-h", "--height"], Store,
            "the height of the window");

        parser.refer(&mut console_args.x_pos)
            .add_option(&["-x"], Store,
            "move the camera on the x axis");
        parser.refer(&mut console_args.y_pos)
            .add_option(&["-y"], Store,
            "move the camera on the y axis");

        parser.refer(&mut console_args.scale)
            .add_option(&["-s", "--scale"], Store,
            "scale the viewport");
        
        parser.refer(&mut console_args.max_itterations)
            .add_option(&["-i", "--itterations"], Store,
            "the maximum number of itterations used");
        
        parser.parse_args_or_exit();
    }

    //setup SDL
    let context = sdl2::init().unwrap();
    let context_video = context.video().unwrap();
    
    let window = context_video.window("Mandelbrot Renderer", console_args.width, console_args.height)
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
    


    let mut pixels = render::render_mandelbrot(&console_args);
    


    let surface = match Surface::from_data(&mut pixels[..], console_args.width, console_args.height, 3 * console_args.width, PixelFormatEnum::RGB24) {
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