pub struct Point {
	pub x: u32,
	pub y: u32,
}
pub struct Surface {
	pub width: usize,
	pub height: usize,
	pub buf: Vec<u32>
}
impl Surface {
	pub fn new(width: usize, height: usize) -> Surface {
		Surface { width: width, height: height, buf: vec![0; width*height] }
	}
	pub fn put(&mut self, p: &Point, c: u32) {
		self.buf[p.y as usize * self.width + p.x as usize] = c;
	}
	pub fn safeput(&mut self, p: &Point, c: u32) {
		if p.x < self.width as u32 && p.y < self.height as u32 {
		self.buf[p.y as usize * self.width + p.x as usize] = c;}
	}
	pub fn getpx(&self, p: &Point) -> u32 {
		if p.x < self.width as u32 && p.y < self.height as u32 {
		return self.buf[p.y as usize * self.width + p.x as usize] as u32;} else {return 0x000000ff};
	}
	pub fn stitch(&mut self, surface: &Surface, corner: &Point) {
		let mut loc: Point;
		for i in 0..surface.height {
			for j in 0..surface.width {
				loc = Point {x: corner.x+j as u32, y: corner.y+i as u32};
				self.safeput(&loc, surface.getpx(&Point { x: j as u32, y: i as u32}));
			}
		}
	}
	pub fn cut(&mut self, p1: &Point, p2: &Point) -> Surface {
		let mut newsurface = Surface::new((p2.x - p1.x) as usize, (p2.y - p1.y) as usize);
		let mut loc: Point;
		for i in 0..newsurface.height {
			for j in 0..newsurface.width {
				loc = Point {x: p1.x+j as u32, y: p1.y+i as u32};
				newsurface.safeput(&Point {x: j as u32, y: i as u32}, self.getpx(&loc));
			}
		}
		newsurface
	}
	pub fn resize(&mut self, w: usize, h: usize) {
		//This resize does not anti-alias. If you want to maintain quality when 
		//downsizing, you'll have to implement your own downsize.
		let mut newsurface = Surface::new(w, h);
		let xtraversal: f32 = (self.width as f32 / w as f32);
		let ytraversal: f32 = (self.height as f32 / h as f32);
		println!("{}", xtraversal);
		let mut loc: Point;
		for i in 0..newsurface.height {
			for j in 0..newsurface.width {
				loc=Point { x: (xtraversal*j as f32).floor() as u32, y: (ytraversal*i as f32).floor() as u32 };
				newsurface.safeput(&Point {x: j as u32, y: i as u32}, self.getpx(&loc));
			}
		}
		self.width = newsurface.width;
		self.height = newsurface.height;
		self.buf = newsurface.buf;
	}
}
