# mandelrust
Renderer for the mandelbrot set, implemented in the rust programming language.

## Implemented features:
- rendering to a window
- commandline interface
- rendering to images

## Planned features:
- multithreading


## Command line options
- -w, --width: Set width of the window in pixes
- -h, --height: Set height of the window in pixels
- -x: The camera offset on the x axis
- -y: The camera offset on the y axis
- -s, --scale: Set the size of the viewport on the x and y axis
- -i, --itterations: The maximum number of itterations per pixel.
- --dont-save: Shows the image inside of a window instead of saving it
- --name: The name of the generated image. This argument has no effect if the --dont-save option is set   

P.S.: Since I am writing this program in order to learn rust, there might be some messy code in this repository ;)

