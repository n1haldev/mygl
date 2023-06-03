use std::fs::OpenOptions;
use std::io::Write;
use std::convert::TryInto;

pub fn init_pixels(width: u32, height: u32) -> Vec<u32> {
    let total_size : usize = (height*width).try_into().unwrap();
    let mut pixels : Vec<u32> = vec![0; total_size];
    pixels
}

pub fn fill(pixels: &mut Vec<u32>, width: u32, height: u32, color: u32) {
    // let mut pixels : Vec<u32> = init_pixels(width, height);
    for i in 0..height {
        for j in 0..width {
            let index = (i*width+j) as usize;
            pixels[index] = color;
        }
    }
}

pub fn fill_rect(pixels: &mut Vec<u32>, width: u32, height: u32, x_start: u32, y_start: u32, x_off: u32, y_off: u32, color: u32) {
    let x_end = x_start + x_off;
    let y_end = y_start + y_off;
    // checking if all x_ and y_ are within bound
    if x_end <= width && y_end <= height {

        for i in y_start..=y_end {
            for j in x_start..=x_end {
                let index = (i*width+j) as usize;
                pixels[index] = color;
            }
        }
    }
    else {
        if x_end > width {
            panic!("x_end has breached maximum x-axis bound");
        }
        if y_end > height {
            panic!("y_end has breached maximum y-axis bound");
        }
    }
} 

pub fn fill_circle(pixels: &mut Vec<u32>, width: u32, height: u32, x_center: u32, y_center: u32, radius: u32, color: u32) {
    let x1 = x_center.saturating_sub(radius);
    let y1 = y_center.saturating_sub(radius);
    let x2 = x_center.saturating_add(radius);
    let y2 = y_center.saturating_add(radius);

    let x_center_i32 = x_center as i32;
    let y_center_i32 = y_center as i32;
    let radius_i32 = radius as i32;

    for i in y1..y2+1 {
        if i > 0 && i < height {
            let y_diff = i as i32 - y_center_i32;
            let y_diff_squared = y_diff * y_diff;

            for j in x1..x2+1 {
                if j > 0 && j < width {
                    let x_diff = j as i32 - x_center_i32;
                    let x_diff_squared = x_diff * x_diff;

                    if x_diff_squared + y_diff_squared <= radius_i32 * radius_i32 {
                        pixels[(i * width + j) as usize] = color;
                    }
                }
            }
        }
    }
}

pub fn draw_line(pixels: &mut Vec<u32>, width: u32, height: u32, x1: u32, y1: u32, x2: u32, y2: u32, color: u32) {
    let mut dx: u32 = 0;
    let mut dy: u32 = 0;
    if x1 > x2 {
        dx = x1 - x2;
    }
    else {
        dx = x2 -x1;
    }
    if y1 > y2 {
        dy = y1 - y2;
    }
    else {
        dy = y2 - y1;
    }
    
    //let dx = x2.saturating_sub(x1);
    //let dy = y2.saturating_sub(y1);

    let mut x_1 = x1;
    let mut x_2 = x2;
    let mut y_1 = y1;
    let mut y_2 = y2;
    if dx != 0 {
        let c = y1 - (dy*x1)/dx;

        if x1 > x2 {
            let mut temp = x_2;
            x_2 = x_1;
            x_1 = temp;
        }
        for x in x_1..=x_2 {
            if x < width {
                let y = (dy*x)/dx + c;
                if y < height {
                    pixels[(y*width + x) as usize] = color;
                }
            }
        }
    }
    else {
        let x = x1;
        if x < width {
            if y1 > y2 {
                let mut temp = y_1;
                y_1 = y_2;
                y_2 = temp;
            }    
            for y in y_1..=y_2 {
                pixels[(y * width + x) as usize] = color;
            }
        }
    }
}

pub fn draw_rect(pixels: &mut Vec<u32>, width: u32, height: u32, x_start: u32, y_start: u32, x_off: u32, y_off: u32, color: u32) {
    let x_end = x_start + x_off;
    let y_end = y_start + y_off;

    if x_end <= width && y_end <= height {
        draw_line(pixels, width, height, x_start, y_start, x_end, y_start, color);
        draw_line(pixels, width, height, x_end, y_start, x_end, y_end, color);
        draw_line(pixels, width, height, x_end, y_end, x_start, y_end, color);
        draw_line(pixels, width, height, x_start, y_end, x_start, y_start, color);
    }
    else {
        panic!("Out of bounds to draw rectangle!");
    }
}

pub fn draw_square(pixels: &mut Vec<u32>, width: u32, height: u32, x_start: u32, y_start: u32, side: u32, color: u32) {
    draw_rect(pixels, width, height, x_start, y_start, side, side, color);
}

pub fn draw_circle(pixels: &mut Vec<u32>, width: u32, height: u32, x_center: u32, y_center: u32, radius: u32, color: u32) {

}

pub fn write_to_ppm(pixels: &Vec<u32>, width: u32, height: u32, file_path: &str) {
    let mut file = match OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file_path) {
            Ok(file) => file,
            Err(error) => {panic!("{}", error);},
        };

    let ppm_header = format!("P6\n{} {} 255\n", width, height);
    if let Err(err) = file.write_all(ppm_header.as_bytes()) {
        panic!("Error writing ppm header, error: {}", err);
    }
    else {
        println!("Header successfully written");
    }

    // adjusting pixels
    // creating a vec to decrease write syscalls
    let mut bytes_vec : Vec<u8> = vec![];
    for i in 0..height {
        for j in 0..width {
            let pixel: u32 = pixels[(i*width+j) as usize];
            let bytes: [u8; 3] = [
                ((pixel>>(8*2))&0xFF) as u8,
                ((pixel>>8)&0xFF) as u8,
                ((pixel)&0xFF) as u8
            ];
            bytes_vec.extend_from_slice(&bytes);

            //if let Err(err) = file.write_all(&bytes) {
            //    panic!("Error writing bytes to file, error: {}", err);
            //}
        }
    }
    if let Err(err) = file.write_all(&bytes_vec) {
        panic!("Error writing bytes to file, error: {}", err);
    }
}
