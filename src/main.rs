/*
 * StegaCean
 *
 * STEGAnography crustaCEAN, steganography program in rust.
 *
 * By: Eoghan West | MIT Licence | Epoche: Oct 21, 2021
 */

use clap::{App, AppSettings, Arg, ArgMatches};

use prgrs::{writeln, Length, Prgrs};
use rgb::*;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;

fn get_args() -> ArgMatches {
    return App::new("StegaCean")
        .version("0.0.1")
        .author("Calacuda. <https://github.com/calacuda>")
        .about("used to encocde and decode hidden text files from pngs.")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            App::new("encode")
                .about("embeds a message in an image file.")
                .version("0.0.1")
                .author("Calacuda. <https://github.com/calacuda>")
                .arg(
                    Arg::new("message")
                        .short('m')
                        .long("message")
                        .value_name("MESSAGE.txt")
                        .about("text file to be hidden.")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("picture")
                        .short('p')
                        .long("picture")
                        .value_name("PICTURE.png")
                        .about("image to hide the message in.")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("OUPUT_FILE")
                        .about("name of output file.")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            App::new("decode")
                .about("pulls a message out of an image file.")
                .version("0.0.1")
                .author("Calacuda. <https://github.com/calacuda>")
                .arg(
                    Arg::new("IMAGE_FILE")
                        .about("name of the input image file.")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("OUPUT_FILE")
                        .about("name of output file.")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();
}

/*
fn read_image(fname: &str) -> ((u32, u32), Vec<u8>){
    /*
    reads in the image.
    */
    let decoder = png::Decoder::new(std::fs::File::open(fname).unwrap());
    let mut reader = decoder.read_info().unwrap();

    let mut idat = vec![0; reader.output_buffer_size().clone()];
    let out_info = reader.next_frame(&mut idat);
    let info = reader.info();
    // println!("{:#?}", info);
    // println!("{:#?}", out_info);

    return (info.size(), idat);
}
*/

fn and(byte: u8, dat: u8) -> u8 {
    return byte & dat;
}

fn or(byte: u8, dat: u8) -> u8 {
    return byte | dat;
}

fn make_byte(byte: u8, encode_one: bool, even: bool) -> u8 {
    /*
    writeln(&format!(
        "make_byte args :  {:?}, {:?}, {:?}",
        byte, encode_one, even,
    ));*/
    return match (encode_one, even) {
        (true, true) => or(byte, 0b0000_0001),
        (false, true) => or(byte, 0b0000_0000),
        (true, false) => and(byte, 0b0000_0001),
        (false, false) => and(byte, 0b0000_0000),
    };
}

/*
fn encode(args: &ArgMatches) -> Result<&str, &str> {
    let needle = args.value_of("message").ok_or("")?;
    let haystack = args.value_of("picture").ok_or("")?;
    let out_fn = args.value_of("OUPUT_FILE").ok_or("")?;
    println!("ecoding   :  {:#?}", needle);
    println!("into file :  {:#?}", haystack);

    let message: Vec<u8> = match fs::read_to_string(needle) {
        Ok(thing) => thing.as_bytes().to_owned(),
        Err(error) => panic!("{}", error),
    };

    let (dim, mut idat) = read_image(haystack);


    let checkers: [u8; 8] = [
        0b1000_0000,
        0b0100_0000,
        0b0010_0000,
        0b0001_0000,
        0b0000_1000,
        0b0000_0100,
        0b0000_0010,
        0b0000_0001,
    ];

    let mut ci = 0;
    let mut c = message[ci];

    for i in Prgrs::new(0..idat.len(), idat.len()) {
        let mut byte: u8 = idat[i].clone();
        if (i % 8 == 0 && ci < message.len() - 1) {
            c = message[ci];
            ci += 1;
        }
        if ci < message.len() - 1 {
            //new_file.push(make_byte(byte, byte & checkers[i % 8] > 0, c % 2 == 0));
            idat[i] = make_byte(byte, byte & checkers[i % 8] > 0, c % 2 == 0);
        } else {
            idat[i] = byte;
        }
        // prgrs::writeln(&format!("{:?}", byte.to_be_bytes()));
        continue;
    }
    println!("data encoded!");
    //println!("{:#?} : {:#?}", &new_file[10..20], &raw_bytes[10..20]);
    // image::save_buffer(out_fn, &new_file, width, height, image::ColorType::Rgb8);
    let out_file = File::create(out_fn).unwrap();
    let ref mut w = BufWriter::new(out_file);
    let mut encoder = png::Encoder::new(w, dim.0, dim.1);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&idat).unwrap();

    Ok(out_fn)
}
*/

fn encode(args: &ArgMatches) -> Result<&str, &str> {
    let needle = args.value_of("message").ok_or("")?;
    let haystack = args.value_of("picture").ok_or("")?;
    let out_fn = args.value_of("OUPUT_FILE").ok_or("")?;

    // println!("ecoding   :  {:#?}", needle);
    // println!("into file :  {:#?}", out_fn);

    let message: Vec<u8> = match fs::read_to_string(needle) {
        Ok(thing) => thing.as_bytes().to_owned(),
        Err(error) => panic!("{}", error),
    };

    let image = match lodepng::decode32_file(haystack) {
        Ok(thing) => thing,
        Err(e) => panic!("{:#?}", e),
    };

    let mut idat: Vec<u8> = image.buffer.as_bytes().to_owned();

    // println!("{:#?}", image);

    let checkers: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];
    let mut mess_i = 0;
    let mut bit_i = 0;
    let mut char = message[mess_i];

    for i in Prgrs::new(0..idat.len(), idat.len()) {
        let mut byte: u8 = idat[i].clone();
        char = message[mess_i];
        bit_i = i % 8;

        if (i % 8 == 0 && mess_i < message.len() - 1 && i != 0) {
            mess_i += 1;
        }

        if mess_i < message.len() - 1 {
            //new_file.push(make_byte(byte, byte & checkers[i % 8] > 0, c % 2 == 0));
            let new_byte = make_byte(
                byte,
                (char as u8 & checkers[i % 8]) == checkers[i % 8],
                byte & 1 == 0,
            );
            if i < 100 {
                writeln(&format!("char {:?}", char as u8));
                writeln(&format!("message index {:?}", mess_i));
                writeln(&format!("is even?  {:?}", char as u8 & checkers[i % 8]));
                writeln(&format!("{:?}", byte & 1 == 0));
                writeln(&format!("new_byte : {:?}", new_byte));
            }
            idat[i] = new_byte;
        } else {
            idat[i] = byte;
        }

        // prgrs::writeln(&format!("{:?}", byte.to_be_bytes()));
        continue;
    }
    println!("data encoded!");
    //println!("{:#?} : {:#?}", &new_file[10..20], &raw_bytes[10..20]);
    // image::save_buffer(out_fn, &new_file, width, height, image::ColorType::Rgb8);
    lodepng::encode32_file(out_fn, &idat, image.width, image.height);

    Ok(out_fn)
}

fn make_u8(bits: [u8; 8]) -> u8 {
    let place_vals = [128, 64, 32, 16, 8, 4, 2, 1];
    let mut sum = 0;

    for i in 0..8 {
        if bits[i] > 0 {
            sum += place_vals[i];
        }
    }

    return sum;
}

fn write_to_file(path: &str, bytes: Vec<u8>) {
    let p = Path::new(path);

    let mut f = match File::create(p) {
        Ok(f) => f,
        Err(e) => panic!("file error: {}", e),
    };

    // for ch in bytes {
    //     let c: char = ch as char;
    //     // f.write(c);
    //     print!("{}  ", c);
    // }

    f.write_all(&bytes);
}

fn decode(args: &ArgMatches) -> Result<&str, &str> {
    let haystack = args.value_of("IMAGE_FILE").ok_or("")?;
    let out_fn = args.value_of("OUPUT_FILE").ok_or("")?;

    // println!("decoding  :  {:#?}", haystack);
    // println!("into file :  {:#?}", out_fn);

    let image = match lodepng::decode32_file(haystack) {
        Ok(thing) => thing,
        Err(e) => panic!("{:#?}", e),
    };

    let img_bytes: Vec<u8> = image.buffer.as_bytes().to_owned();
    let mut new_file: Vec<u8> = Vec::new();
    let mut cur_byte: [u8; 8] = [0; 8];

    for i in Prgrs::new(0..img_bytes.len(), img_bytes.len()) {
        let mod_two = img_bytes[i] % 2;
        cur_byte[i % 8] = mod_two;

        if i < 100 {
            writeln(&format!("byte : {:?}", img_bytes[i]));
            writeln(&format!("mod_two : {:?}", mod_two));
        }
        // writeln(&format!("cur_byte :  {:?}", cur_byte));

        if i % 8 == 7 && i != 0 {
            let dec = make_u8(cur_byte);
            if i < 100 {
                // writeln(&format!("{:?}", cur_byte));
                writeln(&format!("i = {:?}  :  dec = {:?}", i, dec));
            }
            new_file.push(dec);
            //writeln(&format!("{:?}", dec));
            cur_byte = [0; 8];
        }
    }

    write_to_file(out_fn, new_file);

    Ok(out_fn)
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
