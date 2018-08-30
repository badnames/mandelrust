extern crate argparse;
use argparse::{ArgumentParser, Store};

mod window;
mod render;

fn main() {
    let mut console_args = render::RenderArgs {
        width: 800,
        height: 800,
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

    window::start(console_args);
}

