use bresenham::Bresenham;
use crate::types::Point;
use crate::types::Surface;

pub fn refresh(s: &mut Surface, c: u32) {
	for i in s.buf.iter_mut() {
		*i = c;
	}
}

pub fn drawrect(p1: &Point, p2: &Point, color: u32, surface: &mut Surface) {
	for i in 1..((p2.y as i32 - p1.y as i32) as u32) {
		for j in 1..((p2.x as i32 - p1.y as i32) as u32) {
			surface.safeput(&Point {x: j+p1.x, y: i+p1.y}, color);
		}
	}
	//for i in 1..((p2.y as i32 - p1.y as i32) as u32) {
	//	drawline(p1
	//}
}

pub fn drawtri(p1: &Point, p2: &Point, p3: &Point, color: u32, res: u32, surface: &mut Surface) {
	for i in 1..res {
		let np1 = Point { x: average(i as f32/res as f32, p1.x as i32, p3.x as i32) as u32, y: average(i as f32/res as f32, p1.y as i32, p3.y as i32) as u32 };
		let np2 = Point { x: average(i as f32/res as f32, p2.x as i32, p3.x as i32) as u32, y: average(i as f32/res as f32, p2.y as i32, p3.y as i32) as u32 };
		drawline(&np1, &np2, color, surface);
	}
}

pub fn drawline(p1: &Point, p2: &Point, color: u32, surface: &mut Surface) {
	for (x, y) in Bresenham::new((p1.x as isize, p1.y as isize), (p2.x as isize, p2.y as isize)) {
		surface.put(&Point {x: x as u32, y: y as u32}, color)
	}
}

pub fn average(index: f32, n1: i32, n2: i32) -> f32 {
        ((n2-n1) as f32*index)+n1 as f32
}