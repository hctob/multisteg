//use std::env; //eivnornment variables
use std::fs; //file stuff
use std::io; //file io
use std::io::prelude::*;
use std::char;

//const NULL_BYTE: u8 = 0x00;
//const SPACE: u8 = 0x0a;
const LSB_MASK: u8 = 1;

pub fn read_byte_by_byte(file_path: &str) -> Result<Vec<u8>, io::Error> {
    let mut f = fs::File::open(file_path)?;
    let mut bytes = vec![0u8; 0]; //default value of 0 with type u8, capacity 0
    let mut mut_byte_buffer = vec![0u8; 1]; //default value of 1 with type u8, capacity 1

    while f.read(&mut mut_byte_buffer)? != 0 { //read byte by byte through the file we passed in - 0 bytes = EOF
        bytes.extend(&mut_byte_buffer); //takes mutable buffer - also works with vecs
        //bytes.extend appends the bytes to our bytes vector
    }
    Ok(bytes)
}

pub fn validate_header(file_bytes: &Vec<u8>) -> i32 {
    //let ret = -1;
    if &file_bytes[0..2] != "P6".as_bytes() {
        eprintln!("Error: not a valid P6 PPM file.");
        return -1;
    }
    let mut count: i32 = 2;
    let mut space_count: u8 = 0;
    let mut new_lines: u8 = 0;
    loop {
        let byte: u8 = file_bytes[count as usize];

        if count > 25 {
            eprintln!("Error: exceeded expected header count value.");
            return -1;
        }
        if space_count == 1 && new_lines == 3 {
            break;
        }
        else if space_count > 1 || new_lines > 3 {
            eprintln!("Error: exceeded header space or new lines");
            return -1;
        }
        else {
            if byte == 0x0a {
                //if byte is new_line
                new_lines += 1;
            }
            else if byte == 0x20 {
                //if byte is space
                space_count += 1;
            }
        }
        count += 1;
    }
    return count;
}

//https://www.geeksforgeeks.org/reverse-actual-bits-given-number/ used this C function as a reference
fn reverse_bits(mut n: u8) -> u8 {
    let mut rev = 0;
    let mut i = 0;
    loop {
        if i == 8 {
            break;
        }
        else {
            rev <<= LSB_MASK;
            if n & 1 == LSB_MASK {
                rev ^= LSB_MASK;
            }
            n >>= LSB_MASK;
        }
        i += 1
    }
    rev
}

pub fn decode_char(vec: Vec<u8>) -> Result<String, io::Error> {
    let mut res = String::new();
    //let mut char_rep: Vec<u8> = vec![0u8; 0];
    let mut char_rep: [u8; 8] = [0; 8];
    let mut i = 0;
    for bytes in vec {
        let twid = bytes & LSB_MASK;
        char_rep[i] = twid;
        //res += std::string::ToString(twid);
        //let b_char = twid.make_ascii_lowercase();
        i += 1
    }
    let char_rep = &char_rep;
    //println!("{:?}", char_rep);
    //let char_rep: u8 = char_rep.trim().parse().expect("Could not parse to u8");
    let mut char_u8: u8 = 0;
    //let char_u8_2: u8 = 0;
    for i in 0..8 {
        char_u8 |= char_rep[i] << i;
        //char_u8 |= (char_rep[i]) << i;
        //char_u8 |= char_rep[i] | (char_u8 & mask);
    }
    //println!("{:?}", char_rep); //least significant bits represented as an 8 bit char
    //println!("val: {:b}", char_u8); //least significant bits
    //println!("rev: {:b}", reverse_bits(char_u8)); //reveersed LSB for correct value
    res.push(char::from(reverse_bits(char_u8)));
    Ok(res)
}

pub fn decode_message(data_bytes: &Vec<u8>) -> Result<String, io::Error> {
    let mut ascii_representation = String::new();
    let mut i = 0;
    loop {
        let ascii_rep = decode_char(data_bytes[(8*i)..(8*i) + 8].to_vec()).expect("Error: couldn't convert character correctly.");
        if ascii_rep == '\0'.to_string() {
            ascii_representation.push_str(&ascii_rep);
            break;
        }
        ascii_representation.push_str(&ascii_rep);
        i += 1
    }
    Ok(ascii_representation)
}
