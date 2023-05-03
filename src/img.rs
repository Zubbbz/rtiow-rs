use crate::vec::Color;
use png::{BitDepth, ColorType, Encoder, ScaledFloat};
use std::{
	fs::File,
	io::BufWriter,
	path::{Path, PathBuf},
};

pub fn save_image(
	path: Option<PathBuf>,
	res: (u32, u32),
	image_buffer: &[u8],
) -> Result<(), png::EncodingError> {
	// TODO: just gonna keep it as rgb for now, I might make it a parameter later but there isn't really a point
	let file_path = path.unwrap_or(
		std::env::current_dir()
			.unwrap()
			.join(Path::new("./image.png")),
	);
	let file = File::create(file_path).unwrap();

	let w = &mut BufWriter::new(file);
	let mut encoder = Encoder::new(w, res.0, res.1);

	encoder.set_color(ColorType::Rgb);
	encoder.set_depth(BitDepth::Eight);
	encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));

	let mut writer = encoder.write_header().unwrap();

	writer.write_image_data(image_buffer)
}

pub fn push_pixel_color(
	buffer: &mut Vec<u8>,
	color: &mut Color,
	alpha: Option<f64>,
	spp: u32,
) {
	// Divide the color by the number of samples
	let scale = 1.0 / spp as f64;

	color.e[0] *= scale;
	color.e[1] *= scale;
	color.e[2] *= scale;

	buffer.push(float_to_rgb(color.e[0]));
	buffer.push(float_to_rgb(color.e[1]));
	buffer.push(float_to_rgb(color.e[2]));

	if let Some(a) = alpha {
		buffer.push(float_to_rgb(a))
	}
}

// turn a float color into a u8 which is used by the rgb buffer
fn float_to_rgb(f: f64) -> u8 {
	(num::clamp(f, 0.0, 0.999) * 256.0) as u8
}

pub fn lerp_colors(t: f64, colors: (Color, Color)) -> Color {
	(1.0 - t) * colors.0 + t * colors.1
}
