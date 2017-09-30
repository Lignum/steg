extern crate raster;

mod extract;
mod hide;

use std::fs::File;
use std::io::{Read, Write};
use std::env;
use std::error::Error;

fn hide(image_path: &str, file_to_hide_path: &str, output_path: &str) -> Result<usize, std::io::Error> {
    let image = raster::open(image_path).expect("Failed to load image for hiding!");

    let data: Vec<u8> = {
        let mut file = File::open(file_to_hide_path)?;
        let mut buffer = match file.metadata() {
            Ok(m) => Vec::with_capacity(m.len() as usize),
            _ => Vec::new()
        };
        file.read_to_end(&mut buffer)?;
        buffer
    };

    let length = data.len();
    let image = hide::hide_in_image(image, data.as_ref());
    raster::save(&image, output_path).expect("Failed to save image with hidden data!");
    Ok(length)
}

fn extract(image_path: &str, length: usize, output_path: &str) -> Result<(), std::io::Error> {
    let image = raster::open(image_path).expect("Failed to load image for extraction!");
    let mut data = extract::extract_from_image(image, length);

    let mut file = File::create(output_path)?;
    file.write_all(&mut data)
}

fn print_usage(prog_name: &str) {
    eprintln!("Usage: {} [extract <input file> <data size> <output file>]/[hide <image file> <data to hide> <output file>]", prog_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = &args[0];

    if args.len() < 3 {
        print_usage(name);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "extract" => {
            let input_file = &args[2];
            let data_size = match args[3].parse() {
                Ok(i) => i,
                Err(_) => { eprintln!("Entered data size was not a number ({})!", args[2]); return; }
            };
            let output_file = &args[4];

            match extract(input_file.as_str(), data_size, output_file.as_str()) {
                Ok(_) => println!("Successfully extracted to {}", output_file),
                Err(err) => eprintln!("Failed to extract data: {}", err.description())
            }
        },
        "hide" => {
            let input_file = &args[2];
            let image_to_hide_file = &args[3];
            let output_file = &args[4];

            match hide(input_file.as_str(), image_to_hide_file.as_str(), output_file.as_str()) {
                Ok(len) => println!("Successfully hidden {} bytes of data", len),
                Err(err) => eprintln!("Failed to hide data: {}", err.description())
            }
        }
        _ => print_usage(name)
    }
}
