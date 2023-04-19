use crate::vec::Color;
use png::{BitDepth, ColorType, Encoder, ScaledFloat};
use std::{
	fs::File,
	io::BufWriter,
	path::{Path, PathBuf},
};

pub struct ImageRes {
	pub width: u32,
	pub height: u32,
}

pub fn save_image(
	path: Option<PathBuf>,
	res: ImageRes,
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
	let mut encoder = Encoder::new(w, res.width, res.height);

	encoder.set_color(ColorType::Rgb);
	encoder.set_depth(BitDepth::Eight);
	encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));

	let mut writer = encoder.write_header().unwrap();

	writer.write_image_data(image_buffer)
}

pub fn push_pixel_color(
	buffer: &mut Vec<u8>,
	color: Color,
	alpha: Option<f64>,
) {
	buffer.push(num::clamp(255.999 * color.x(), 0.0, 255.0) as u8);
	buffer.push(num::clamp(255.999 * color.y(), 0.0, 255.0) as u8);
	buffer.push(num::clamp(255.999 * color.z(), 0.0, 255.0) as u8);

	match alpha {
		Some(a) => buffer.push(num::clamp(255.999 * a, 0.0, 255.0) as u8),
		None => (),
	}
}
