use crate::{ray::Ray, vec::Color};
use png::{BitDepth, ColorType, Encoder, ScaledFloat};
use std::{
	fs::File,
	io::BufWriter,
	path::{Path, PathBuf},
};

pub fn save_image(
	path: Option<PathBuf>,
	res: (u32, u32),
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
	let mut encoder = Encoder::new(w, res.0, res.1);

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
	buffer.push(float_to_rgb(color.x()));
	buffer.push(float_to_rgb(color.y()));
	buffer.push(float_to_rgb(color.z()));

	match alpha {
		Some(a) => buffer.push(num::clamp(255.999 * a, 0.0, 255.0) as u8),
		None => (),
	}
}

// turn a float color into a u8 which is used by the rgb buffer
fn float_to_rgb(f: f64) -> u8 {
	num::clamp(255.999 * f, 0.0, 255.0) as u8
}

fn lerp_colors(t: f64, colors: (Color, Color)) -> Color {
	(1.0 - t) * colors.0 + t * colors.1
}

pub fn ray_color(ray: &Ray, colors: Option<(Color, Color)>) -> Color {
	let normalized_ray_dir = ray.direction.normalize();
	let t: f64 = 0.5 * (normalized_ray_dir.y() + 1.0);
	let colors = colors.unwrap_or((
		Color { e: [1.0, 1.0, 1.0] },
		Color { e: [0.5, 0.7, 1.0] },
	));
	lerp_colors(t, colors)
}
