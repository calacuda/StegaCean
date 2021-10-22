/*
 * StegaCean
 *
 * STEGAnography crustaCEAN, steganography program in rust.
 *
 * By: Eoghan West | MIT Licence | Epoche: Oct 21, 2021
 */


use clap::{
    Arg,
    App,
    ArgMatches,
    AppSettings
};
//use image;
use image::io::Reader as ImageReader;
use prgrs::{Prgrs, writeln, Length};
//use std::io::Bytes;
use std::fs;
use std::str;
use std::net::SocketAddr;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


fn get_args() -> ArgMatches {
    return App::new("StegaCean")
        .version("0.0.1")
	.author("Calacuda. <https://github.com/calacuda>")
        .about("used to encocde and decode hidden text files from pngs.")
	.setting(AppSettings::SubcommandRequired)
        .subcommand(App::new("encode")
		    .about("embeds a message in an image file.")
		    .version("0.0.1")
		    .author("Calacuda. <https://github.com/calacuda>")
		    .arg(Arg::new("message")
			 .short('m')
			 .long("message")
			 .value_name("MESSAGE.txt")
			 .about("text file to be hidden.")
			 .takes_value(true)
			 .required(true))
		    .arg(Arg::new("picture")
			 .short('p')
			 .long("picture")
			 .value_name("PICTURE.png")
			 .about("image to hide the message in.")
			 .takes_value(true)
			 .required(true))
		    .arg(Arg::new("OUPUT_FILE")
			 .about("name of output file.")
			 .required(true)
			 .index(1)))
	.subcommand(App::new("decode")
		    .about("pulls a message out of an image file.")
		    .version("0.0.1")
		    .author("Calacuda. <https://github.com/calacuda>")
		    .arg(Arg::new("IMAGE_FILE")
			 .about("name of the input image file.")
			 .required(true)
			 .index(1))
		    .arg(Arg::new("OUPUT_FILE")
			 .about("name of output file.")
			 .required(true)
			 .index(2)))
	.get_matches();
}

fn read_image(fname: &str) -> Option<Vec<u8>> {
    let raw_img = ImageReader::open(fname).ok()?.decode().ok()?;
    return Some(raw_img.into_rgb8().to_vec());
}

fn and(byte: u8, dat:u8) -> u8 {
    return byte & dat;
}

fn or(byte: u8, dat:u8) -> u8 {
    return byte | dat;
}

fn make_byte(byte: u8, encode_one: bool, even:bool) -> u8 {
    return match (encode_one, even) {
	(true, true) => or(byte, 0b0000_0001),
	(false, true) => or(byte, 0b0000_0000),
	(true, false) => and(byte, 0b0000_0001),
	(false, false) => and(byte, 0b0000_0000),
    };
}

fn get_dim(path: &str) -> (u32, u32) {
    match image::image_dimensions(path) {
	Ok(val) => val,
	Err(mes) => panic!("{}", mes), 
    }
}

fn encode(args: &ArgMatches) -> Result<&str, &str> {
    let needle = args.value_of("message").ok_or("")?;
    let haystack = args.value_of("picture").ok_or("")?;
    let out_file = args.value_of("OUPUT_FILE").ok_or("")?;
    println!("ecoding   :  {:#?}", needle);
    println!("into file :  {:#?}", haystack);

    let checkers: [u8; 8] = [0b1000_0000,
			     0b0100_0000,
			     0b0010_0000,
			     0b0001_0000,
			     0b0000_1000,
			     0b0000_0100,
			     0b0000_0010,
			     0b0000_0001];
    let message: Vec<u8> = match fs::read_to_string(needle) {
	Ok(thing) => thing.as_bytes().to_owned(),
	Err(error) => panic!("{}", error),
    };
    let raw_bytes = read_image(haystack).unwrap();
    let (width, height) = get_dim(haystack);
    let mut new_file: Vec<u8> = Vec::new();
    let mut ci = 0;
    let mut c = message[ci];
    
    for i in Prgrs::new(0..raw_bytes.len(), raw_bytes.len()) {
	let mut byte: u8 = raw_bytes[i].clone();
	if (i%8 == 0 && ci < message.len() - 1) {
	    ci += 1;
	    c = message[ci];
	}
	if ci < message.len() - 1 {
	    new_file.push(make_byte(byte, byte & checkers[i%8] > 0, c%2 == 0));
	}
	else {
	    new_file.push(byte);
	}
	// prgrs::writeln(&format!("{:?}", byte.to_be_bytes()));
	continue
    }
    println!("data encoded!");
    //println!("{:#?} : {:#?}", &new_file[10..20], &raw_bytes[10..20]);
    image::save_buffer(out_file, &new_file, width, height, image::ColorType::Rgb8);
    
    Ok(out_file)
}

fn make_u8(bits: [u8; 8]) -> u8 {
    let place_vals = [128, 64, 32, 16, 8, 4, 2, 1];
    let mut sum = 0;
    
    for i in 0..8 {
	if bits[i] > 0 {
	    sum += place_vals[i];
	}
    }

    return sum
}

fn write_to_file(path: &str, bytes: Vec<u8>) {
    let p = Path::new(path);

    let mut f = match File::create(p) {
	Ok(f) => f,
	Err(e) => panic!("file error: {}", e),
    };
    /*
    for ch in bytes {
	//let c: char = ch as char;
	//f.write(c);
	//print!("{}  ", ch);
    }
     */
    f.write_all(&bytes);
}

fn decode(args: &ArgMatches) -> Result<&str, &str> {
    let haystack = args.value_of("IMAGE_FILE").ok_or("")?;
    let out_file = args.value_of("OUPUT_FILE").ok_or("")?;

    let img_bytes = read_image(haystack).unwrap();
    let mut new_file: Vec<u8> = Vec::new();
    let mut cur_byte: [u8; 8] = [0; 8];

    for i in Prgrs::new(0..img_bytes.len(), img_bytes.len()) {
	cur_byte[i%8] = img_bytes[i] & 0b0000_0001;
	
	if i%8 == 7 && i != 0 {
	    new_file.push(make_u8(cur_byte));
	    cur_byte = [0; 8];
	}
    }

    write_to_file(out_file, new_file);

    Ok(out_file)
}

fn main() {
    let matches = get_args();
    let args = match matches.subcommand() {
	Some(thing) => thing,
	None => panic!("no subcommand given"),
    };

    let result = match args.0 {
	"encode" => encode(args.1),
	"decode" => decode(args.1),
	_ => Err("teh hobbits to isengard!"),
    };

    match result {
	Ok(fname) => println!("data writen to <{}>", fname),
	Err(_) => println!("Error, try again."),
    };
    
}
