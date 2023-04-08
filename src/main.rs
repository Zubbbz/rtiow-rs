use std::{
	fs::File,
	io::{BufWriter},
	path::Path,
};

use png::{BitDepth, ColorType, Encoder, ScaledFloat};

const IMG_WIDTH: u32 = 512;
const IMG_HEIGHT: u32 = 512;

fn main() {
	let mut rgb_buffer: Vec<u8> = vec![];

	for y in (0..IMG_HEIGHT).rev() {
		// eprint!("\rLines Remaining: {} ", y);
		// std::io::stderr().flush().unwrap();

		for x in 0..IMG_HEIGHT {
			let r = (x as f64) / (IMG_WIDTH - 1) as f64;
			let g = (y as f64) / (IMG_HEIGHT - 1) as f64;
			let b = 0.25f64;

			rgb_buffer.push((255.999 * r) as u8);
			rgb_buffer.push((255.999 * g) as u8);
			rgb_buffer.push((255.999 * b) as u8);
		}
	}

	let path = std::env::current_dir()
		.unwrap()
		.join(Path::new("./image.png"));
	let file = File::create(path).unwrap();
	let ref mut w = BufWriter::new(file);
	let mut encoder = Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);
	encoder.set_color(ColorType::Rgb);
	encoder.set_depth(BitDepth::Eight);
	encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));
	let mut writer = encoder.write_header().unwrap();
	writer.write_image_data(&rgb_buffer).unwrap();

	println!("\nDone.\n");
}
