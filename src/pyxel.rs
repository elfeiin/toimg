// The Pyxel. Purposely spelled with a Y to differentiate it from normal pixels.
#[derive(Debug, Copy, Clone)]
pub struct Pyxel {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
	x: usize,
	y: usize,
}

impl Pyxel {
	// Returns a new Pyxel. Takes an array of 4 u8s and two usizes as args
	pub fn new(color: [u8; 4], x: usize, y: usize) -> Pyxel {
		Pyxel {
			r: color[0],
			g: color[1],
			b: color[2],
			a: color[3],
			x: x,
			y: y,
		}
	}

	// Returns the channels formatted as an array of u8
	pub fn color_u8(&self) -> [u8; 4] {
		[self.r(), self.g(), self.b(), self.a()]
	}

	// Returns the channels formatted as an array of f32
	pub fn color_f32(&self) -> [f32; 4] {
		[
			self.r() as f32 / 255.0,
			self.g() as f32 / 255.0,
			self.b() as f32 / 255.0,
			self.a() as f32 / 255.0,
		]
	}

	// Returns respective values
	pub fn r(&self) -> u8 {
		self.r
	}

	pub fn g(&self) -> u8 {
		self.g
	}

	pub fn b(&self) -> u8 {
		self.b
	}

	pub fn a(&self) -> u8 {
		self.a
	}

	pub fn x(&self) -> usize {
		self.x
	}

	pub fn y(&self) -> usize {
		self.y
	}
}

pub struct PyxelVec {
	pyxels: Vec<Pyxel>,
	width: usize,
	height: usize,
}

impl PyxelVec {
	pub fn new(width: usize, height: usize) -> PyxelVec {
		PyxelVec {
			pyxels: vec!(),
			width: width,
			height: height,
		}
	}
	
	pub fn push(&mut self, pyxel: Pyxel) {
		self.pyxels.push(pyxel);
	}
	
	pub fn to_vec_u8(&self, dark: bool) -> Vec<u8> {
		let mut v = vec!();
		if dark {
			v = vec![0u8; self.width * self.height * 3];
		} else {
			v = vec![255u8; self.width * self.height * 3];
		}
		for p in self.pyxels.iter() {
			let r = p.r() as f32 / 255.0;
			let g = p.g() as f32 / 255.0;
			let b = p.b() as f32 / 255.0;
			let a = p.a() as f32 / 255.0;
			
			let or = v[((p.y() * self.width * 3) + p.x() * 3) + 0] as f32 / 255.0;
			let og = v[((p.y() * self.width * 3) + p.x() * 3) + 1] as f32 / 255.0;
			let ob = v[((p.y() * self.width * 3) + p.x() * 3) + 2] as f32 / 255.0;
			
			let nr = ((a * r + (1.0 - a) * or) * 255.0) as u8;
			let ng = ((a * g + (1.0 - a) * og) * 255.0) as u8;
			let nb = ((a * b + (1.0 - a) * ob) * 255.0) as u8;
			
			v[((p.y() * self.width * 3) + p.x() * 3) + 0] = nr;
			v[((p.y() * self.width * 3) + p.x() * 3) + 1] = ng;
			v[((p.y() * self.width * 3) + p.x() * 3) + 2] = nb;
		}
		v
	}
}