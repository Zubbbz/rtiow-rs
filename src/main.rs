use std::{
	fs::File,
	io::{BufWriter, Write},
	path::{Path, PathBuf},
};

use png::{BitDepth, ColorType, Encoder, ScaledFloat};

const IMG_WIDTH: u32 = 512;
const IMG_HEIGHT: u32 = 512;

fn main() {
	let mut rgb_buffer: Vec<u8> = vec![];

	for y in (0..IMG_HEIGHT).rev() {
		eprint!("\rLines Remaining: {} ", y);
		std::io::stderr().flush().unwrap();

		for x in 0..IMG_WIDTH {
			let r = (x as f64) / (IMG_WIDTH - 1) as f64;
			let g = (y as f64) / (IMG_HEIGHT - 1) as f64;
			let b = 0.25;

			push_pixel_color(&mut rgb_buffer, r, g, b, None);
		}
	}

	save_image(None, &rgb_buffer).unwrap();
	println!("\nDone.\n");
}

fn save_image(
	path: Option<PathBuf>,
	image_buffer: &Vec<u8>,
) -> Result<(), png::EncodingError> {
	// TODO: just gonna keep it as rgb for now, I might make it a parameter later but there isn't really a point
	let file_path = path.unwrap_or(
		std::env::current_dir()
			.unwrap()
			.join(Path::new("./image.png")),
	);
	let file = File::create(file_path).unwrap();

	let ref mut w = BufWriter::new(file);
	let mut encoder = Encoder::new(w, IMG_WIDTH, IMG_HEIGHT);

	encoder.set_color(ColorType::Rgb);
	encoder.set_depth(BitDepth::Eight);
	encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));

	let mut writer = encoder.write_header().unwrap();

	writer.write_image_data(image_buffer)
}

fn push_pixel_color(
	buffer: &mut Vec<u8>,
	red: f64,
	green: f64,
	blue: f64,
	alpha: Option<f64>,
) {
	buffer.push(num::clamp(255.999 * red, 0.0, 255.0) as u8);
	buffer.push(num::clamp(255.999 * green, 0.0, 255.0) as u8);
	buffer.push(num::clamp(255.999 * blue, 0.0, 255.0) as u8);

	match alpha {
		Some(a) => buffer.push(num::clamp(255.999 * a, 0.0, 255.0) as u8),
		None => (),
	}
}
