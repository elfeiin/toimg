use super::pointer::*;
use super::pyxel::*;

// The Command struct
#[derive(Debug, Clone)]
pub struct Command {
	verb: char,
	hex: [u8; 4],
	int: isize,
	rep: usize,
	unset: bool,
}

impl Command {
	// Returns a new Command. Takes 5 arguments.
	pub fn new(verb: char, hex: [u8; 4], int: isize, rep: usize, unset: bool) -> Self {
		Command {
			verb,
			hex,
			int,
			rep,
			unset,
		}
	}

	// Returns verb. The command.
	pub fn verb(&self) -> &char {
		&self.verb
	}

	// Returns hex. A color.
	pub fn hex(&self) -> [u8; 4] {
		self.hex
	}

	// Returns int. This is used to move the pointer around in various directions.
	pub fn int(&self) -> isize {
		self.int
	}

	// Returns rep. This is used to repeat various commands.
	pub fn rep(&self) -> usize {
		self.rep
	}

	// Returns unset. This is used to detect when to set things like pointer start to a default value.
	pub fn unset(&self) -> bool {
		self.unset
	}
}

// My sort of iterator
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Looper {
	index: usize,
	remaining: usize,
}

impl Looper {
	// Returns the index of the looper
	pub fn index(&self) -> usize {
		self.index
	}

	// Returns how many iterations are remaining
	pub fn remaining(&self) -> usize {
		self.remaining
	}

	// Decrements the remaining
	pub fn decrease(&mut self) {
		self.remaining -= 1;
	}
}

// Run the commands and modify the Tab's Pyxels
pub fn run(pointer: &mut Pointer, comms: Vec<Command>) -> PyxelVec {
	let mut pyxels: PyxelVec = PyxelVec::new(pointer.width(), pointer.height());
	let mut i = 0;
	let mut repeat_table: Vec<Looper> = vec![];
	
	while i < comms.len() {
		pointer.check_pos();
		let cmd = &comms[i];
		match cmd.verb() {
			'^' => {
				pointer.slide(0, -cmd.int());
			}

			'v' => {
				pointer.slide(0, cmd.int());
			}
			
			'<' => {
				pointer.slide(-cmd.int(), 0);
			}
			
			'>' => {
				pointer.slide(cmd.int(), 0);
			}

			'n' => {
				pointer.slide(0, 1);
				pointer.set_pos(if pointer.reverse_move_x() {
					pointer.right()
				} else {
					pointer.left()
				}, pointer.y());
			}

			's' => {
				if cmd.unset() {
					pointer.set_virtual_left(0);
				} else {
					pointer.set_virtual_left(cmd.int());
				}
			}

			'e' => {
				if cmd.unset() {
					pointer.set_virtual_right(pointer.width() as isize);
				} else {
					pointer.set_virtual_right(cmd.int());
				}
			}

			'S' => {
				if cmd.unset() {
					pointer.set_virtual_top(0);
				} else {
					pointer.set_virtual_top(cmd.int());
				}
			}

			'E' => {
				if cmd.unset() {
					pointer.set_virtual_bottom(pointer.height() as isize);
				} else {
					pointer.set_virtual_bottom(cmd.int());
				}
			}

			'X' => {
				pointer.flip_reverse_move_x();
			}

			'Y' => {
				pointer.flop_reverse_move_y();
			}

			'[' => {
				repeat_table.push(Looper {
					index: i,
					remaining: cmd.rep() - 1,
				});
			}

			// Check for any Loopers and set i to their jump value if we have any
			// Decrement their jump times
			']' => match repeat_table.last() {
				Some(_) => {
					let ilen = repeat_table.len() - 1;
					let last = &mut repeat_table[ilen];
					if last.remaining() > 0 {
						i = last.index();
						last.decrease();
					} else {
						repeat_table.remove(ilen);
					}
				}
				_ => (),
			}

			'#' => {
				let mut k = 0;
				while k < cmd.rep() {
					pyxels.push(Pyxel::new(
						cmd.hex(),
						pointer.x().abs() as usize,
						pointer.y().abs() as usize,
					));
					pointer.slide(1, 0);
					k += 1;
				}
			}

			_ => ()
		}

		i += 1;
	}
	pyxels
}
