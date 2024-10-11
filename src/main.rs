use std::{env, fs};
use std::io::{Cursor};
use std::path::{Path};
use image::{ImageReader};

fn filestr_to_enum(file_type: &str) -> image::ImageFormat {
    match file_type {
        "jpg" => image::ImageFormat::Jpeg,
        "jpeg" => image::ImageFormat::Jpeg,
        "png" => image::ImageFormat::Png,
        _ => {image::ImageFormat::Jpeg}
    }
}

fn convert_file(path: &Path, file_type: &String) {
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let format = filestr_to_enum(file_type.as_str());
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let new_file_extension = ".".to_owned() + &*file_type.to_string();
    let new_file_name = file_name.replace(".webp", &*new_file_extension);
    img.save(&new_file_name).expect("saving file failed");

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), format).unwrap();

    let img2 = ImageReader::new(Cursor::<&mut Vec<u8>>::new(bytes.as_mut())).with_guessed_format().unwrap().decode().unwrap();
    img2.write_to(&mut Cursor::new(&mut bytes), format).unwrap();

    println!("image converted");
}

fn traverse_files(path: &Path, total_files: &mut u32, file_type: &String) {
    let files = fs::read_dir(path).unwrap();
    if path.is_dir() {
        println!("Scanning directory {}", path.display().to_string());
        for file in files {
            let file = file.unwrap();
            let path = file.path();
            if path.is_dir() {
                traverse_files(&path, total_files, file_type);
            } else if path.extension().unwrap().to_str().unwrap() == file_type.to_string() {
                *total_files += 1;
                convert_file(&path, &file_type);
            }
        }
    }
}

fn main() {
    let flag: String = env::args().nth(1).unwrap();
    let file_type: String = env::args().nth(2).unwrap();
    let path: &Path = Path::new(&flag);
    let mut total_webp_files = 0;
    traverse_files(&path, &mut total_webp_files, &file_type);
    println!("\n{} webp files in {} converted", total_webp_files, path.display().to_string());
}
