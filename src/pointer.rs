// Not a memory pointer (I mean not technically but you get it)
// Points to a position on a Canvas
#[derive(Debug, Clone)]
pub struct Pointer {
	x: isize,
	y: isize,
	reverse_move_x: bool,
	reverse_move_y: bool,
	width: usize,
	height: usize,
	top: isize,
	bottom: isize,
	left: isize,
	right: isize,
}

impl Pointer {
	// Returns a new pointer. Takes no arguments.
	pub fn new() -> Self {
		Pointer {
			x: 0,
			y: 0,
			reverse_move_x: false,
			reverse_move_y: false,
			width: 32,
			height: 32,
			top: 0,
			bottom: 32,
			left: 0,
			right: 32,
		}
	}

	// Returns respective values smh
	pub fn x(&self) -> isize {
		self.x
	}

	pub fn y(&self) -> isize {
		self.y
	}

	pub fn reverse_move_x(&self) -> bool {
		self.reverse_move_x
	}

	pub fn reverse_move_y(&self) -> bool {
		self.reverse_move_y
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.height
	}

	pub fn top(&self) -> isize {
		self.top
	}

	pub fn bottom(&self) -> isize {
		self.bottom
	}

	pub fn left(&self) -> isize {
		self.left
	}

	pub fn right(&self) -> isize {
		self.right
	}

	pub fn set_pos(&mut self, r: isize, d: isize) {
		self.x = r;
		self.y = d;
	}

	pub fn check_pos(&mut self) {
		let mut x = self.x();
		let mut y = self.y();
		
		if x < self.left() {
			if self.reverse_move_x() {
				x = self.right() - 1;
			} else {
				x = self.left();
			}
		}
		if x >= self.right() {
			if self.reverse_move_x() {
				x = self.right();
			} else {
				x = self.left();
			}
		}
		
		if y < self.top() {
			y = self.bottom() - 1;
		}
		if y >= self.bottom() {
			y = self.top();
		}
		self.set_pos(x, y);
	}

	pub fn move_pos(&mut self, r: isize, d: isize) {
		let mut r_unit = 0;
		if r != 0 {
			r_unit = r/r.abs();
		}
		let r_unit = r_unit;
		let mut d_unit = 0;
		if d != 0 {
			d_unit = d/d.abs();
		}
		let d_unit = d_unit;
		let mut x = self.x();
		let mut y = self.y();
		
		for i in 0..r.abs() {
			let t_r = x + r_unit;
			if t_r >= self.left() && t_r < self.right() {
				x += r_unit;
			}
			if t_r < self.left() {
				if self.reverse_move_x() {
					x = self.right() - 1;
					if self.reverse_move_y() {
						y -= 1;
					} else {
						y += 1;
					}
				} else {
					x = self.left();
				}
			}
			if t_r >= self.right() {
				if self.reverse_move_x() {
					x = self.right() - 1;
				} else {
					x = self.left();
					if self.reverse_move_y() {
						y -= 1;
					} else {
						y += 1;
					}
				}
			}
		}
		
		for i in 0..d.abs() {
			let t_d = y + d_unit;
			if t_d >= self.top() && t_d < self.bottom() {
				y += d_unit;
			}
			if t_d < self.top() {
				y = self.bottom() - 1;
			}
			if t_d >= self.bottom() {
				y = self.top();
			}
		}
		self.set_pos(x, y);
	}

	pub fn slide(&mut self, r: isize, d: isize) {
		let mut r = r;
		let mut d = d;
		if self.reverse_move_x() {
			r = -r;
		}
		if self.reverse_move_y() {
			d = -d;
		}
		self.move_pos(r, d);
	}

	pub fn flip_reverse_move_x(&mut self) {
		self.reverse_move_x = !self.reverse_move_x;
	}

	// Purposely spelled differently just to make things difficult
	pub fn flop_reverse_move_y(&mut self) {
		self.reverse_move_y = !self.reverse_move_y;
	}

	pub fn set_virtual_left(&mut self, n: isize) {
		self.left = n;
		self.x = n;
	}

	pub fn set_virtual_right(&mut self, n: isize) {
		self.right = self.width() as isize - n;
	}

	pub fn set_virtual_top(&mut self, n: isize) {
		self.top = n;
		self.y = n;
	}

	pub fn set_virtual_bottom(&mut self, n: isize) {
		self.bottom = self.height() as isize - n;
	}

	pub fn set_width(&mut self, w: usize) {
		let mut w = w;
		if w == 0 {
			w = 1;
		}
		self.width = w;
		self.set_virtual_right(0);
	}

	pub fn set_height(&mut self, h: usize) {
		let mut h = h;
		if h == 0 {
			h = 1;
		}
		self.height = h;
		self.set_virtual_bottom(0);
	}

	// Blank the pointer! >:D
	pub fn blank(&mut self, w: usize, h: usize) {
		self.x = 0;
		self.y = 0;
		self.reverse_move_x = false;
		self.reverse_move_y = false;
		self.top = 0;
		self.width = w;
		self.height = h;
		self.bottom = self.height() as isize;
		self.left = 0;
		self.right = self.width() as isize;
	}
}
