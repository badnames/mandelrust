pub fn render_mandelbrot(width: u32, height: u32, x_pos: f64, y_pos: f64, scale: f64, max_itterations: u32) -> Vec<u8> {

    let array_size = (3 * width * height) as usize;
    
    let mut pixels: Vec<u8> = vec![0xFF; array_size];

    println!("rendering...");

    for y in 0..height {
        
        for x in 0..width {
            let mut x_transformed: f64;
            let mut y_transformed: f64;

            x_transformed = (x as f64) / ((width) as f64)  * scale - scale / 2.0 + x_pos;
            y_transformed = (y as f64) / ((height) as f64) * scale - scale / 2.0 + y_pos;
        
            let itterations = itterate(x_transformed, y_transformed, max_itterations);

            if itterations == max_itterations {
                pixels[coordinates_to_array_index(width, x, y) + 0] = 0x00; //RED
                pixels[coordinates_to_array_index(width, x, y) + 1] = 0x00; //GREEN
                pixels[coordinates_to_array_index(width, x, y) + 2] = 0x00; //BLUE
                continue;
            }
        }
    }

    println!("done");

    pixels
}

fn itterate(x: f64, y: f64, max_itterations: u32) -> u32 {
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

fn coordinates_to_array_index(width: u32, x: u32, y: u32) -> usize {
    ((y * width * 3) + (x * 3)) as usize
}