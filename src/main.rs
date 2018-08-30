extern crate argparse;
extern crate image;

use argparse::{ArgumentParser, Store, StoreFalse};

mod window;
mod render;

fn main() {
    let mut render_args = render::RenderArgs {
        width: 800,
        height: 800,
        x_pos: 0.0,
        y_pos: 0.0,
        scale: 5.0,
        max_itterations: 50
    };

    let mut generate_image = true;
    let mut image_name     = "mandelbrot.png".to_string();

    //parse commandline arguments
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Renders a slice of the mandelbrot set.");
        
        parser.refer(&mut render_args.width)
            .add_option(&["-w", "--width"], Store,
            "the width of the window");
        parser.refer(&mut render_args.height)
            .add_option(&["-h", "--height"], Store,
            "the height of the window");

        parser.refer(&mut render_args.x_pos)
            .add_option(&["-x"], Store,
            "move the camera on the x axis");
        parser.refer(&mut render_args.y_pos)
            .add_option(&["-y"], Store,
            "move the camera on the y axis");

        parser.refer(&mut render_args.scale)
            .add_option(&["-s", "--scale"], Store,
            "scale the viewport");
        
        parser.refer(&mut render_args.max_itterations)
            .add_option(&["-i", "--itterations"], Store,
            "the maximum number of itterations used");
        
        parser.refer(&mut generate_image)
            .add_option(&["--dont-save"], StoreFalse,
            "render to a window instead of an image");
        
        parser.refer(&mut image_name)
            .add_option(&["--name"], Store,
            "the name of the generated image");

        parser.parse_args_or_exit();
    }

    if generate_image {
        let buffer = render::render_mandelbrot(&render_args);

        image::save_buffer("image.png", &buffer[..], render_args.width, render_args.height, image::RGB(8)).unwrap();
    } else {
        window::start(&render_args);
    }
}

