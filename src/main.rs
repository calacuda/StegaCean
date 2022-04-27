/*
 * StegaCean
 *
 * STEGAnography crustaCEAN, steganography program in rust.
 *
 * By: Eoghan West | MIT Licence | Epoche: Oct 21, 2021
 */

use clap::{App, AppSettings, Arg, ArgMatches};
use lodepng;
use rgb::*;
use std::fs;
use std::fs::File;
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

fn encode(args: &ArgMatches) -> Result<&str, &str> {
    let needle = args.value_of("message").ok_or("")?;
    let haystack = args.value_of("picture").ok_or("")?;
    let out_fn = args.value_of("OUPUT_FILE").ok_or("")?;

    println!("ecoding   :  {:#?}", needle);
    println!("into file :  {:#?}", out_fn);

    let message: Vec<u8> = match fs::read_to_string(needle) {
        Ok(thing) => thing.as_bytes().to_owned(),
        Err(error) => panic!("{}", error),
    };

    let image = match lodepng::decode32_file(haystack) {
        Ok(thing) => thing,
        Err(e) => panic!("{:#?}", e),
    };

    let mut idat: Vec<u8> = image.buffer.as_bytes().to_owned();

    // input message as a mit sequence
    let mut message_bits: Vec<bool> = Vec::new();

    // turns the input message into a sequence of bits
    // should change this to a map to increas efficiency.
    for c in &message {
        add_bits(&c, &mut message_bits);
    }
    // adds EOT char to the end of the sequence
    add_bits(&0b0000_0100, &mut message_bits);

    // let mut idat_i = 3;

    // encode the data
    for i in 0..message_bits.len() {
        let bit: bool = message_bits[i];
        idat[i] = if bit { idat[i] | 1 } else { idat[i] & 254 };
        // idat_i += 4;
    }

    // println!("data encoded!");
    match lodepng::encode32_file(out_fn, &idat, image.width, image.height) {
        Ok(_) => {}
        Err(e) => panic!("error while writing pixel data to file {}", e),
    };

    Ok(out_fn)
}

fn add_bits(num: &u8, bits: &mut Vec<bool>) {
    /*
     * used to make a giant vector of bits of all the chars from teh text file
     */
    let place_vals = [128, 64, 32, 16, 8, 4, 2, 1];

    for pv in place_vals {
        if num & pv != 0 {
            bits.push(true);
        } else {
            bits.push(false);
        }
    }
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

    match f.write_all(&bytes) {
        Ok(_) => {}
        Err(e) => panic!("error while writing text to file: {}", e),
    };
}

fn decode(args: &ArgMatches) -> Result<&str, &str> {
    let haystack = args.value_of("IMAGE_FILE").ok_or("")?;
    let out_fn = args.value_of("OUPUT_FILE").ok_or("")?;

    println!("decoding  :  {:#?}", haystack);
    println!("into file :  {:#?}", out_fn);

    let image = match lodepng::decode32_file(haystack) {
        Ok(thing) => thing,
        Err(e) => panic!("{:#?}", e),
    };

    let img_bytes: Vec<u8> = image.buffer.as_bytes().to_owned();
    let mut new_file: Vec<u8> = Vec::new();
    let mut cur_byte: [u8; 8] = [0; 8];

    for i in 0..img_bytes.len() {
        let mod_two = img_bytes[i] % 2;
        cur_byte[i % 8] = mod_two;

        if i % 8 == 7 && i != 0 {
            let dec = make_u8(cur_byte);

            if dec == 4 {
                // this breaks when we encounter the first EOT character
                break;
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
        _ => Err("there was an error while taking the hobbits to isengard!"),
    };

    match result {
        Ok(fname) => println!("data writen to <{}>", fname),
        Err(_) => println!("Error, try again."),
    };
}
