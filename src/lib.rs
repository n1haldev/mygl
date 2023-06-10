use std::fs::OpenOptions;
use std::io::Write;
use std::convert::TryInto;

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

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

pub fn draw_line(pixels: &mut Vec<u32>, width: u32, height: u32, x1: u32, y1: u32, x2: u32, y2: u32, color: u32, thickness: u32) {
    let dx: i32 = x2 as i32 - x1 as i32;
    let dy: i32 = y2 as i32 - y1 as i32;

    for thick in 0..=thickness {
    let mut x_1 = x1 + thick;
    let mut x_2 = x2 + thick;
    let mut y_1 = y1 + thick;
    let mut y_2 = y2 + thick;

    if dx != 0 {
        let c = y_1 as i32 - (dy*x_1 as i32)/dx;
        if x1 > x2 {
            let mut temp = x_1;
            x_1 = x_2;
            x_2 = temp;
        }
        for x in x_1..=x_2 {
            if x > 0 && x < width {
                let mut sy1 = dy*x as i32/dx + c;
                let mut sy2 = dy*(x as i32 + 1)/dx + c;
                if sy1 > sy2 {
                    let mut temp = sy1;
                    sy1 = sy2;
                    sy2 = temp;
                }
                for y in sy1..=sy2 {
                    if y > 0 && y < height as i32 {
                        pixels[(y*width as i32 + x as i32) as usize] = color; 
                    }
                }
            }
        }
    }

    else {
        let x = x1 + thick;
        if x < width {
            if y1 > y2 {
                let mut temp = y_1;
                y_1 = y_2;
                y_2 = temp;
            }
            for y in y_1..y_2 {
                pixels[(y as i32*width as i32 + x as i32) as usize] = color;
            }
        }
    }
    }
}

pub fn draw_rect(pixels: &mut Vec<u32>, width: u32, height: u32, x_start: u32, y_start: u32, x_off: u32, y_off: u32, color: u32, thickness: u32) {
    let x_end = x_start + x_off;
    let y_end = y_start + y_off;

    for thick in 0..=thickness {
    if x_start-thick > 0 && y_start-thick > 0 && x_end+thick <= width && y_end+thick <= height {
        draw_line(pixels, width, height, x_start-thick, y_start-thick, x_end+thick, y_start-thick, color, 1);
        draw_line(pixels, width, height, x_end+thick, y_start-thick, x_end+thick, y_end+thick, color, 1);
        draw_line(pixels, width, height, x_end+thick, y_end+thick, x_start-thick, y_end+thick, color, 1);
        draw_line(pixels, width, height, x_start-thick, y_end+thick, x_start-thick, y_start-thick, color, 1);
    }
    else {
        panic!("Out of bounds to draw rectangle!");
    }
    }
}

pub fn draw_square(pixels: &mut Vec<u32>, width: u32, height: u32, x_start: u32, y_start: u32, side: u32, color: u32, thickness: u32) {
    draw_rect(pixels, width, height, x_start, y_start, side, side, color, thickness);
}

pub fn draw_circle(pixels: &mut Vec<u32>, width: u32, height: u32, x_center: u32, y_center: u32, radius: u32, color: u32, thickness: u32) {
    let x1 = x_center.saturating_sub(radius);
    let x2 = x_center.saturating_add(radius);
    let y1 = y_center.saturating_sub(radius);
    let y2 = y_center.saturating_add(radius);

    let x_center_i32 = x_center as i32;
    let y_center_i32 = y_center as i32;
    let radius_i32 = radius as i32;

    for i in x1..=x2 {
        if i >= 0 && i<= width {
            let x_diff = i as i32 - x_center_i32;
            let x_diff_squared = x_diff * x_diff;
            for j in y1..=y2 {
                if j >= 0 && j<= height {
                    let y_diff = j as i32 - y_center_i32;
                    let y_diff_squared = y_diff * y_diff;
                    let distance = f64::sqrt((x_diff_squared + y_diff_squared) as f64);
                    if distance >= (radius_i32-thickness as i32) as f64 && distance <= radius_i32 as f64 {
                        pixels[(j*width + i) as usize] = color;
                    }
                }
            }
        }
    }
}

// function does not work properly yet
/*
pub fn draw_sector(pixels: &mut Vec<u32>, width: u32, height: u32, x_center: u32, y_center: u32, radius: u32, start_angle: f32, end_angle: f32, color: u32) {
    let angle_diff = end_angle - start_angle;
    let angle_step = 0.01;
    let x_center_f32 = x_center as f32;
    let y_center_f32 = y_center as f32;
    let radius_f32 = radius as f32;
    let start_angle_rad = start_angle.to_radians();
    let end_angle_rad = end_angle.to_radians();

    for angle in (start_angle_rad..=end_angle_rad).step_by(angle_step as usize) {
        let x = x_center_f32 + radius_f32 * angle.cos();
        let y = y_center_f32 + radius_f32 * angle.sin();

        let x1 = x.round() as u32;
        let y1 = y.round() as u32;

        if x1 >= 0 && x1 < width && y1 >= 0 && y1 < height {
            pixels[(y1 * width + x1) as usize] = color;
        }
    }
}
*/

fn sort_triangle_points(x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32) -> ((u32, u32), (u32, u32), (u32, u32)) {
    if y1 > y2 {
        let (x1, x2) = swap(x1, x2);
        let (y1, y2) = swap(y1, y2);
    }
    if y2 > y3 {
        let (x2, x3) = swap(x2, x3);
        let (y2, y3) = swap(y2, y3);
    }
    if y1 > y2 {
        let (x1, x2) = swap(x1, x2);
        let (y1, y2) = swap(y1, y2);
    }
    ((x1, y1), (x2, y2), (x3, y3))
}

fn line_info(x1: u32, y1: u32, x2: u32, y2: u32, k: &mut u32, c: &mut u32) {
    let (mut dy, mut dx) = (0,0);
    if y1 > y2 {
        dy = y1 - y2;
    }
    else {
        dy = y2 - y1;
    }
    
    if x1 > x2 {
        dx = x1 - x2;
    }
    else {
        dx = x2 - x1;
    }

    if dx != 0 {
        *k = dy/dx;
        *c = y1 - dy*x1/dx;
    }
}

pub fn fill_triangle(pixels: &mut Vec<u32>, width: u32, height: u32, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, color: u32) {
    let (p1, p2, p3) = sort_triangle_points(x1, y1, x2, y2, x3, y3);
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    let (x3, y3) = p3;

    let (mut k12, mut c12, mut k23, mut c23, mut k13, mut c13) = (0,0,0,0,0,0);
    line_info(x1, y1, x2, y2, &mut k12, &mut c12);
    line_info(x2, y2, x3, y3, &mut k23, &mut c23);
    line_info(x1, y1, x3, y3, &mut k13, &mut c13);
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
