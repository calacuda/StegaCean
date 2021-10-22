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
use std::net::SocketAddr;
use std::error::Error;


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

fn read_image(fname: &str) -> Option<image::RgbImage> {
    let raw_img = ImageReader::open(fname).ok()?.decode().ok()?;
    return Some(raw_img.into_rgb8());
}

/*
fn set_low_byte(number:u8, byte:u8) -> u8 {
    let mut arr = number.to_be_bytes();
    //writeln(&format!("{:#?}", arr));
    return arr
    //arr[3] = byte;
    //return u8::from_be_bytes(arr)
}
 */

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

fn encode(args: &ArgMatches) -> Result<(), &str> {
    let needle = args.value_of("message").ok_or("")?;
    let haystack = args.value_of("picture").ok_or("")?;
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
    let raw_bytes = read_image(haystack).unwrap().to_vec();
    let mut new_file: Vec<u8> = Vec::new();
    let mut ci = 0;
    let mut c = message[ci];
    
    for i in Prgrs::new(0..raw_bytes.len(), raw_bytes.len()) {
	let mut byte: u8 = raw_bytes[i].clone();
	if (i%8 == 0 && ci < message.len() - 1) {
	    ci += 1;
	    c = message[ci];
	}
	new_file.push(make_byte(byte, byte & checkers[i%8] > 0, c%2 == 0));
	// prgrs::writeln(&format!("{:?}", byte.to_be_bytes()));
	continue
    }
    println!("data encoded!");
    //println!("{:#?}", new_file);
    let mut same = true;
    for i in Prgrs::new(0..raw_bytes.len(), raw_bytes.len()) {
	same = same && (new_file[i] == raw_bytes[i]); 
    }
    println!("files are same {}", same);
    
    //let out_img = ImageReader::new(Cursor::new(bytes)).decode()?;
    
    Ok(())
}

fn decode(args: &ArgMatches) -> Result<(), &str> {
    let file = args.value_of("IMAGE_FILE").ok_or("")?;
    println!("{:#?}", file);
    Ok(())
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
	Ok(_) => println!("done!"),
	Err(_) => println!("Error, try again."),
    };
    
}
