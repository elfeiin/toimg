use super::cmd::*;
use meval;

pub fn parse(s: &String) -> Option<(usize, usize, Vec<Command>)> {
	let mut parser = Parser::new();
	parser.parse(s)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tone {
	Light,
	Normal,
	Dark,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Comment {
	Nope,
	Line,
	Mult,
}

#[derive(Debug, Clone)]
struct Parser {
	cmd: char,
	hex: String,
	num: String,
	sharps: bool,
	tone: Tone,
	comment: Comment,
	cmds: Vec<Command>,
}

impl Parser {
	fn new() -> Parser {
		Parser {
			cmd: char::from(0u8),
			hex: String::new(),
			num: String::new(),
			sharps: false,
			tone: Tone::Normal,
			comment: Comment::Nope,
			cmds: vec![],
		}
	}

	// Return respective values
	fn cmd(&self) -> char {
		self.cmd
	}

	fn hex(&self) -> String {
		self.hex.to_owned()
	}

	fn num(&self) -> String {
		self.num.to_owned()
	}

	fn sharps(&self) -> bool {
		self.sharps
	}

	fn tone(&self) -> Tone {
		self.tone.to_owned()
	}

	fn comment(&self) -> Comment {
		self.comment.to_owned()
	}

	fn cmds(&self) -> Vec<Command> {
		self.cmds.to_owned()
	}

	// Resets the parser
	fn reset(&mut self) {
		self.cmd = char::from(0u8);
		self.hex = String::new();
		self.num = String::new();
		self.sharps = false;
		self.tone = Tone::Normal;
	}

	// Set respective values
	fn set_cmd(&mut self, a: char) {
		self.cmd = a;
	}

	fn set_hex(&mut self, a: String) {
		self.hex = a;
	}

	fn set_num(&mut self, a: String) {
		self.num = a;
	}

	fn push_hex(&mut self, a: char) {
		self.hex.push(a);
	}

	fn push_num(&mut self, a: char) {
		self.num.push(a);
	}

	fn set_sharps(&mut self, a: bool) {
		self.sharps = a;
	}

	fn set_tone(&mut self, a: Tone) {
		self.tone = a;
	}

	fn set_comment(&mut self, a: Comment) {
		self.comment = a;
	}

	// Add a command into the list of commands
	fn put(&mut self) {
		if self.cmd() != char::from(0u8) {
			let int = parse_num(self.num()).0;
			let rep = parse_num(self.num()).1;
			if self.sharps {
				self.cmds.push(Command::new(
					self.cmd(),
					parse_hex(self.hex()),
					int,
					rep,
					self.hex().len() == 0 && self.num().len() == 0,
				));
			} else {
				self.cmds.push(Command::new(
					self.cmd(),
					tone(self.hex(), self.tone()),
					int,
					rep,
					self.hex().len() == 0 && self.num().len() == 0,
				));
			}
		}
	}

	// Takes a string and converts it to commands
	fn parse(&mut self, s: &String) -> Option<(usize, usize, Vec<Command>)> {
		let mut width = 32usize;
		let mut height = 32usize;
		let mut lines: Vec<&str> = s.lines().collect();
		if lines.len() > 1 {
			width = match lines[0].parse::<usize>() {
				Ok(n) => n,
				Err(e) => 32,
			};
			height = match lines[1].parse::<usize>() {
				Ok(n) => n,
				Err(e) => 32,
			};
		} else {
			return None;
		}
		lines.remove(0);
		lines.remove(0);
		let mut s = String::new();
		for line in lines {
			for c in line.chars() {
				s.push(c);
			}
			s.push('\n');
		}
		
		
		self.cmds = vec![];
		self.reset();
		let num_list: String = String::from("0123456789-+/*");
		let hex_list: String = String::from("0123456789abcdef");
		let color_list: String = String::from("rgbcymw.");
		let control_list = String::from("^v<>[]esESXY");
		let mut chars = s.chars();
		while let Some(c) = chars.next() {
			match c {
				'|' => {
					if self.comment() == Comment::Nope {
						self.set_comment(Comment::Line);
					}
					self.reset();
				}

				'{' => {
					if self.comment() == Comment::Nope {
						self.set_comment(Comment::Mult);
					}
					self.reset();
				}

				'}' => {
					self.set_comment(Comment::Nope);
					self.reset();
				}

				' ' => {
					// self.put();
					// self.reset();
				}

				'\n' => {
					if self.comment() == Comment::Nope {
						self.set_cmd('n');
						self.put();
					} else {
						if self.comment() == Comment::Line {
							self.set_comment(Comment::Nope);
						}
					}
					self.reset();
				}

				_ => (),
			}

			if self.comment == Comment::Nope {
				if self.sharps() {
					if let Some(_) = hex_list.find(c) {
						self.push_hex(c);
						continue;
					}
				} else {
					if let Some(_) = num_list.find(c) {
						self.push_num(c);
						continue;
					}
				}
				match c {
					'l' => {
						self.set_tone(Tone::Light);
					}
					'd' => {
						self.set_tone(Tone::Dark);
					}
					'#' => {
						self.set_cmd('#');
						self.set_sharps(true);
					}
					_ => (),
				}
				if let Some(_) = color_list.find(c) {
					self.set_cmd('#');
					self.set_hex(c.to_string());
					self.put();
					self.reset();
				}
				if let Some(_) = control_list.find(c) {
					self.set_cmd(c);
					self.put();
					self.reset();
				}
			}
		}

		self.put();
		self.reset();
		Some((width, height, self.cmds()))
	}
}

// Convert a string containing a valid hex value to an array of u8
fn parse_hex(s: String) -> [u8; 4] {
	let mut r: u8 = 255;
	let mut g: u8 = 255;
	let mut b: u8 = 255;
	let mut a: u8 = 255;

	// Uhhh.....
	match s.len() {
		3 => {
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[..1], &s[..1]), 16) {
				r = n;
			}
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[1..2], &s[1..2]), 16) {
				g = n;
			}
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[2..], &s[2..]), 16) {
				b = n;
			}
		}
		6 => {
			if let Ok(n) = u8::from_str_radix(&s[..2], 16) {
				r = n;
			}
			if let Ok(n) = u8::from_str_radix(&s[2..4], 16) {
				g = n;
			}
			if let Ok(n) = u8::from_str_radix(&s[4..], 16) {
				b = n;
			}
		}
		4 => {
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[..1], &s[..1]), 16) {
				r = n;
			}
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[1..2], &s[1..2]), 16) {
				g = n;
			}
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[2..3], &s[2..3]), 16) {
				b = n;
			}
			if let Ok(n) = u8::from_str_radix(&format!("{}{}", &s[3..], &s[3..]), 16) {
				a = n;
			}
		}
		8 => {
			if let Ok(n) = u8::from_str_radix(&s[..2], 16) {
				r = n;
			}
			if let Ok(n) = u8::from_str_radix(&s[2..4], 16) {
				g = n;
			}
			if let Ok(n) = u8::from_str_radix(&s[4..6], 16) {
				b = n;
			}
			if let Ok(n) = u8::from_str_radix(&s[6..], 16) {
				a = n;
			}
		}
		_ => (),
	}

	[r, g, b, a] // Return all our variables, formatted in an array
}

// Returns a tuple containing the conversion of a string to an isize and the same string to a usize, respectively
fn parse_num(s: String) -> (isize, usize) {
	let mut i: isize = 0;
	let mut u: usize = 1;
	match meval::eval_str(s) {
		Ok(n) => {
			i = n as isize;
			u = n.abs() as usize;
		}
		_ => (),
	}

	(i, u)
}

// Makes a color darker or lighter or does nothing to it
fn tone(s: String, l: Tone) -> [u8; 4] {
	let mut max: u8 = 255;
	let mut min: u8 = 0;

	match l {
		Tone::Light => {
			min = 192;
		}
		Tone::Dark => {
			max = 192;
		}
		_ => (),
	}

	match s.as_ref() {
		"r" => [max, min, min, 255],
		"y" => [max, max, min, 255],
		"g" => [min, max, min, 255],
		"c" => [min, max, max, 255],
		"b" => [min, min, max, 255],
		"m" => [max, min, max, 255],
		"w" => [max, max, max, 255],
		_ => [min, min, min, 255],
	}
}
