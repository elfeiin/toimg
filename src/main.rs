mod cmd;
use cmd::*;
mod parser;
use parser::*;
mod pointer;
use pointer::*;
mod pyxel;
use pyxel::*;

use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use image;

fn display_help() {
	println!("Usage: toimg [OPTION]... [FILE]...\n
Converts text to image and saves as an image file in the current directory.\n
Default format is BMP.\n
 -h, --help                   Displays this help.
 -f, --format [FORMAT]        The format to save the image in (default: .bmp). Available formats:
                                 .png
                                 .bmp
                                 .jpg
                                 .ico
                                 .tif
 -n, --name [NAME]            The name of the output file (else saves as [FILE] name). Will overwrite the file if it already exists.
     --doc                    Display information on how to write input files.");
}

fn display_doc() {
	println!("toimg: Takes as input a text file and converts it to a pixel image.
The first line of the input file is a number that determines the width of the output image.
The second line is the height.

To repeat most commands, put a number before it (denoted n; default: 1):
 2r
 
Otherwise, this number serves as an argument:
 4s -> start at 4
 5[r] -> repeat put red pyxel 5 times
 
Supports mathematical operations:
 2 + 3r
 4*2b
 
Can be negative:
 -2> -> moves pointer to the left 2 pyxels
 
Invalid characters are ignored.

Command    Description:
 |       | Single line comment.
 {{ ... }} | Multiline comment.
 [ ... ] | Creates a loop that repeats n times. Can be nested.
 #       | Starts a 3 or 4 byte hex color code.
         |    Ex: #fff #ffffff #f008 #ff000088.
 l       | Make the next command light.
 d       | Make the next command dark.
 r       | Puts n red pyxels.
 g       | Puts n green pyxels.
 b       | Puts n blue pyxels.
 c       | Puts n cyan pyxels.
 y       | Puts n yellow pyxels.
 m       | Puts n magenta pyxels.
 w       | Puts n white pyxels.
 .       | Puts n black pyxels.
 ^       | Moves the pointer up n pyxels.
 v       | Moves the pointer down n pyxels.
 <       | Moves the pointer left n pyxels.
 >       | Moves the pointer right n pyxels.
 e       | Sets pointer to skip last n pyxels of every line or none if no number given.
 s       | Sets pointer to skip first n pyxels of every line or none if no number given.
 E       | Sets pointer to skip last n pyxels of every column or none if no number given.
 S       | Sets pointer to skip first n pyxels of every column or none if no number given.
 X       | Reverses the horizontal direction the pointer moves.
 Y       | Reverses the vertical direction the pointer moves.
 \\n      | Moves the pointer down and resets pointer x");
}

fn main() -> std::io::Result<()> {
	let curr_dir = env::current_dir()?;
	let mut first = true;
	
	let mut do_format = false;
	let mut format = String::from(".bmp");
	let mut do_name = false;
	let mut name = String::new();
	let mut dark: bool = false;
	let mut dir = PathBuf::new();
	
	for arg in env::args() {
		if first {
			first = false;
			continue;
		}
		
		if arg.starts_with("-") {
			match arg.as_ref() {
				"--help" | "-h" => {
					display_help();
					return Ok(());
				},
				"--doc" => {
					display_doc();
					return Ok(());
				},
				"--format" | "-f" => do_format = true,
				"--name" | "-n" => do_name = true,
				"--dark" => dark = true,
				_ => {
					println!("toimg: Invalid option: {}", arg);
					println!("Try 'toimg --help' for more information.");
					return Ok(());
				},
			}
		} else if arg.starts_with("./") {
			dir.push(&curr_dir);
			let sub: String = arg.chars().skip(2).take(arg.len()).collect::<String>();
			dir.push(sub);
		} else if arg.starts_with("../") {
			dir.push(&curr_dir);
			dir.pop();
			let sub: String = arg.chars().skip(3).take(arg.len()).collect::<String>();
			dir.push(sub);
		} else if arg.starts_with("..") {
			dir.push(&curr_dir);
			dir.pop();
			let sub: String = arg.chars().skip(2).take(arg.len()).collect::<String>();
			dir.push(sub);
		} else if arg.starts_with("/") {
			dir.push(arg);
		} else {
			if !do_format && !do_name {
				dir.push(&curr_dir);
				dir.push(arg);
			} else {
				let arg_clone = arg.clone();
				if do_format {
					do_format = false;
					format = match arg_clone.to_ascii_lowercase().as_ref() {
						"png"          => ".png".to_string(),
						"bmp"          => ".bmp".to_string(),
						"jpeg" | "jpg" => ".jpg".to_string(),
						"icon" | "ico" => ".ico".to_string(),
						"tiff" | "tif" => ".tif".to_string(),
						_              => {
							println!("toimg: Invalid format: {}", arg_clone);
							println!("Try 'toimg --help' for more information.");
							return Ok(());
						}
					}
				}
				let arg_clone = arg.clone();
				if do_name {
					name = arg_clone;
				}
			}
		}
	}
	
	if name.len() == 0 {
		let mut dir_clone = dir.clone();
		name = dir_clone.to_str().unwrap().split("/").into_iter().last().unwrap().to_string();
	}
	name = format!("{}{}", name, format);
	
	let colortype = image::ColorType::RGB(8);
	
	let dir_clone = dir.clone();
	match File::open(dir_clone) {
		Ok(file) => {
			let mut buf_reader = BufReader::new(file);
			let mut contents = String::new();
			match buf_reader.read_to_string(&mut contents) {
				Ok(_)	 => {
					if let Some((w, h, cmds)) = parser::parse(&contents) {
						let mut pointer = pointer::Pointer::new();
						pointer.set_width(w);
						pointer.set_height(h);
						let pyxels = cmd::run(&mut pointer, cmds);
						let vu8 = pyxels.to_vec_u8(dark);
						dir.pop();
						dir.push(name);
						let mut image = File::create(dir)?;
						let width = pointer.width();
						let height = pointer.height();
						match format.as_ref() {
							".png"     => {
								let mut encoder = image::png::PNGEncoder::new(&mut image);
								encoder.encode(&vu8, width as u32, height as u32, colortype);
							},
							".jpg"     => {
								let mut encoder = image::jpeg::JPEGEncoder::new(&mut image);
								encoder.encode(&vu8, width as u32, height as u32, colortype);
							},
							".ico"     => {
								let mut encoder = image::ico::ICOEncoder::new(&mut image);
								encoder.encode(&vu8, width as u32, height as u32, colortype);
							},
							".tif"     => {
								let mut encoder = image::tiff::TiffEncoder::new(&mut image);
								encoder.encode(&vu8, width as u32, height as u32, colortype);
							},
							".bmp" | _ => {
								let mut encoder = image::bmp::BMPEncoder::new(&mut image);
								encoder.encode(&vu8, width as u32, height as u32, colortype);
							},
						};
						image.sync_all()?;
					}
				},
				Err(e) => {
					println!("Could not read {}: {}", dir.display(), e);
					println!("Try 'toimg --help' for more information.");
					return Ok(());
				}
			}
		},
		Err(e) => {
			println!("Could not open {}: {}", dir.display(), e);
			println!("Try 'toimg --help' for more information.");
			return Ok(());
		}
	}
	
	Ok(())
}