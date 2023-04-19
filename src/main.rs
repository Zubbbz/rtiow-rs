use rtiow::{
	img::{push_pixel_color, save_image, ImageRes},
	vec::Color,
};
use std::io::Write;

const RES: ImageRes = ImageRes {
	width: 256,
	height: 256,
};

fn main() {
	let mut rgb_buffer: Vec<u8> = vec![];

	for y in (0..RES.height).rev() {
		eprint!("\rLines Remaining: {} ", y);
		std::io::stderr().flush().unwrap();

		for x in 0..RES.width {
			push_pixel_color(
				&mut rgb_buffer,
				Color {
					e: [
						(x as f64) / (RES.width - 1) as f64,
						(y as f64) / (RES.width - 1) as f64,
						0.25,
					],
				},
				None,
			);
		}
	}

	save_image(None, RES, &rgb_buffer).unwrap();
	println!("\nDone.\n");
}
