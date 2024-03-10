use minifb::{Key, Window, WindowOptions};

pub mod graphics;
pub mod types;
use crate::types::*;
use crate::graphics::*;
use std::time::Instant;
fn main() {
    let mut s = Surface::new(513, 513);
    let mut tst = Surface::new(512, 512);
    drawtri(&Point {x: 0, y: 0}, &Point {x: 511, y: 0}, &Point {x: 0, y: 511}, 0x00ff0000, 512, &mut tst);
    drawtri(&Point {x: 511, y: 511}, &Point {x: 0, y: 511}, &Point {x: 511, y: 0}, 0x000000ff, 512, &mut tst);
    let mut test = tst.cut(&Point {x: 0, y: 0}, &Point {x: 512, y: 512});
    test.resize(16, 16);
    test.resize(256, 256);

    let mut window = Window::new(
        "Test - ESC to exit",
        s.width,
        s.height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); //60 FPS
    let mut ftest = 0;
    let mut fcount = 0;
    let mut fmeasure = Instant::now();
    let mut framerate = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
	//Framerate testing code
	fmeasure = if ftest == 0 {Instant::now()} else {fmeasure};
	fcount += if ftest == 1 {1} else {-fcount};
	ftest = if ftest == 0 {1} else {1};
	framerate = if fmeasure.elapsed().as_secs() >= 1 {fcount} else {framerate};
	ftest = if fmeasure.elapsed().as_secs() >= 1 {0} else {1};
        refresh(&mut s, 0x00ff00);
	s.stitch(&test, &Point { x: 0, y: 0});
        window
            .update_with_buffer(&s.buf, s.width, s.height)
            .unwrap();
    }
}
