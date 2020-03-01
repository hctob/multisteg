use std::env; //eivnornment variables
//use std::path;
use std::str::FromStr;
use multisteglib;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() - 1 {
        0 => {
            eprintln!("Usage: \nDecoding: cargo run <threads> <directory>\nEncoding: cargo run ");
        },
        2 => {
            /*println!("Two arguments");
            let args_slice = &args[1].trim();
            let file_bytes: Vec<u8> = multisteglib::read_byte_by_byte(args_slice).expect("Error: could not read file bytes.");
            let valid_header: i32 = multisteglib::validate_header(&file_bytes);
            if valid_header == -1 {
                eprintln!("Error: invalid header type.");
            }
            else {
                let valid_header: usize = valid_header as usize;
                println!("Header length: {}", valid_header);
                let data_bytes: Vec<u8> = file_bytes[valid_header..].to_vec();
                let decoded_message = multisteglib::decode_message(&data_bytes).unwrap();
                println!("{}", decoded_message);
            }*/
            //working decode for one image
            let _threads: u32 = u32::from_str(&args[1].trim()).unwrap();
            let decode_directory = &args[2].trim();

            let directory_check = std::path::Path::new(decode_directory).metadata().expect("Error: could not find path to specified directory");
            assert!(directory_check.is_dir()); //assert that argument is actually a directory
            //let dir = std::path::Path::new(decode_directory);
            //assert!(std::env::set_current_dir(&dir).is_ok());
            //println!("Successfully set working directory to {:?}", dir.display());
            let mut ret: String = String::new();
            //iterate through each picture in the directory
            for file in std::fs::read_dir(decode_directory).expect("Error: could not iterate through directory contents") {
                let dir = file.expect("Error: could not check file path");
                //println!("{:?}", dir.path());
                let file_path = dir.path();
                assert!(file_path.is_file()); //assert that file is a file
                let file_name: &str = file_path.file_name().unwrap().to_str().unwrap();

                println!("{}", file_name);
                let file_bytes: Vec<u8> = multisteglib::read_byte_by_byte(file_path.to_str().unwrap()).unwrap();
                let valid_header = multisteglib::validate_header(&file_bytes);
                if valid_header == -1 {
                    //eprintln!("{:?} is not a PPM file", dir.path());
                    continue;
                }
                else {
                    //for each valid PPM file
                    let valid_header = valid_header as usize;
                    let data_bytes: Vec<u8> = file_bytes[valid_header..].to_vec();
                    ret = multisteglib::decode_message(&data_bytes).unwrap();
                }
            }
            println!("{}", ret);
        },
        4 => {
            println!("Four arguments");
        },
        _ => {
            eprintln!("Usage: \nDecoding: cargo run <threads> <directory>\nEncoding: cargo run ");
        }
    }
}
