use std::io::Write;

use rtiow::*;

const RES: ImageRes = ImageRes {
	width: 512,
	height: 512,
};

fn main() {
	let mut rgb_buffer: Vec<u8> = vec![];

	for y in (0..RES.height).rev() {
		eprint!("\rLines Remaining: {} ", y);
		std::io::stderr().flush().unwrap();

		for x in 0..RES.width {
			let r = (x as f64) / (RES.width - 1) as f64;
			let g = (y as f64) / (RES.width - 1) as f64;
			let b = 0.25;

			push_pixel_color(&mut rgb_buffer, r, g, b, None);
		}
	}

	save_image(None, RES, &rgb_buffer).unwrap();
	println!("\nDone.\n");
}
