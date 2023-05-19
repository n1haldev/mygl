use std::fs::OpenOptions;
use std::io::Write;
use std::convert::TryInto;

// defining colours as global variables


pub fn init_pixels(width: u32, height: u32) -> Vec<u32> {
    let total_size : usize = (height*width).try_into().unwrap();
    let mut pixels : Vec<u32> = vec![0; total_size];
    pixels
}

pub fn fill_rect(width: u32, height: u32, color: u32) -> Vec<u32> {
    let mut pixels : Vec<u32> = init_pixels(width, height);
    for i in 0..height {
        for j in 0..width {
            let index = (i*width+j) as usize;
            pixels[index]=color;
        }
    }
    pixels
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
