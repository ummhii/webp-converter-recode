use std::{env, fs};
use std::io::{Cursor};
use std::path::{Path};
use image::{ImageReader};

fn convert_file(path: &Path) {
    // TODO
    let img = ImageReader::open(path).unwrap().decode().unwrap();

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let new_file_name = file_name.replace(".webp", ".jpg");
    img.save(&new_file_name).expect("saving file failed");

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg).unwrap();

    let img2 = ImageReader::new(Cursor::new(bytes.as_ref())).with_guessed_format().unwrap().decode().unwrap();
    img2.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg).unwrap();

    println!("image converted");
}

fn traverse_files(path: &Path, total_files: &mut u32) {
    let files = fs::read_dir(path).unwrap();
    if path.is_dir() {
        println!("Scanning directory {}", path.display().to_string());
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            if path.is_dir() {
                traverse_files(&path, total_files);
            } else if path.extension().unwrap() == "webp" {
                *total_files += 1;
                convert_file(&path);
            }
        }
    }
}

fn main() {
    let flag: String = env::args().nth(1).unwrap();
    let path: &Path = Path::new(&flag);
    let mut total_webp_files = 0;
    traverse_files(&path, &mut total_webp_files);
    println!("\n{} webp files in {} converted", total_webp_files, path.display().to_string());
}
