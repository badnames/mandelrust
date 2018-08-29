use std::thread;

pub fn render_mandelbrot(width: u32, height: u32, x_pos: f64, y_pos: f64, scale: f64, max_itterations: u32) -> Vec<u8> {

    let array_size = (3 * width * height) as usize;
    
    let mut pixels: Vec<u8> = vec![0xFF; array_size];

    println!("rendering...");

    let screen_coords_to_world_coords = |screen_coord: f64, screen_side_length: u32, scale: f64, camera_pos: f64| {
        (screen_coord) / (screen_side_length as f64)  * scale - scale / 2.0 + camera_pos
    };

    for y in 0..height {
        
        for x in 0..width {

            let mut x1_transformed: f64 = screen_coords_to_world_coords((x as f64) - 0.5, width,  scale, x_pos);
            let mut y1_transformed: f64 = screen_coords_to_world_coords((y as f64) - 0.5, height, scale, y_pos);

            let mut subpixel1: [u32; 3] = [0x00, 0x00, 0x00];
            let handle1 = thread::spawn(move || {
                subpixel1 = calculate_subpixel(x1_transformed, y1_transformed, max_itterations);
            });
            

            let mut x2_transformed: f64 = screen_coords_to_world_coords((x as f64) + 0.5, width,  scale, x_pos);
            let mut y2_transformed: f64 = screen_coords_to_world_coords((y as f64) - 0.5, height, scale, y_pos);

            let mut subpixel2: [u32; 3] = [0x00, 0x00, 0x00];
            let handle2 = thread::spawn(move || {
                subpixel2 = calculate_subpixel(x2_transformed, y2_transformed, max_itterations);
            });

            let mut x3_transformed: f64 = screen_coords_to_world_coords((x as f64) - 0.5, width,  scale, x_pos);
            let mut y3_transformed: f64 = screen_coords_to_world_coords((y as f64) + 0.5, height, scale, y_pos);

            let mut subpixel3: [u32; 3] = [0x00, 0x00, 0x00];
            let handle3 = thread::spawn(move || {
                subpixel3 = calculate_subpixel(x3_transformed, y3_transformed, max_itterations);
            });


            let mut x4_transformed: f64 = screen_coords_to_world_coords((x as f64) + 0.5, width,  scale, x_pos);
            let mut y4_transformed: f64 = screen_coords_to_world_coords((y as f64) + 0.5, height, scale, y_pos);

            let mut subpixel4: [u32; 3] = [0x00, 0x00, 0x00];
            let handle4 = thread::spawn(move || {
                subpixel4 = calculate_subpixel(x4_transformed, y4_transformed, max_itterations);
            });

            handle1.join().unwrap();
            handle2.join().unwrap();
            handle3.join().unwrap();
            handle4.join().unwrap();

            pixels[coordinates_to_array_index(width, x, y) + 0] = ((subpixel1[0] + subpixel2[0] + subpixel3[0] + subpixel4[0]) / 4) as u8; //RED
            pixels[coordinates_to_array_index(width, x, y) + 1] = ((subpixel1[1] + subpixel2[1] + subpixel3[1] + subpixel4[1]) / 4) as u8; //GREEN
            pixels[coordinates_to_array_index(width, x, y) + 2] = ((subpixel1[2] + subpixel2[2] + subpixel3[2] + subpixel4[2]) / 4) as u8; //BLUE
        
        }
    }

    println!("done");

    pixels
}

fn calculate_subpixel(x_transformed: f64, y_transformed: f64, max_itterations: u32) -> [u32; 3]{

    let itterations = itterate(x_transformed, y_transformed, max_itterations);

    if(itterations == max_itterations) {
        return [0x00, 0x00, 0x00];
    }

    return [0xFF, 0xFF, 0xFF]
}



fn itterate(cx: f64, cy: f64, max_itterations: u32) -> u32 {
    let mut zx: f64 = 0.0;
    let mut zy: f64 = 0.0;

    for itteration in 1..max_itterations {
        let xx = zx * zx;
        let yy = zy * zy;
        let xy = zx * zy;

        zx = (xx - yy) + cx;
        zy = 2.0 * xy + cy;

        if (zx * zx + zy * zy) > 4.0 {
            return itteration;
        }  
    }

    max_itterations

}

fn coordinates_to_array_index(width: u32, x: u32, y: u32) -> usize {
    ((y * width * 3) + (x * 3)) as usize
}