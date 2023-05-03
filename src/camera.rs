use crate::{
	ray::Ray,
	vec::{Point3, Vec3},
};

pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
}

impl Camera {
	pub fn new(
		aspect_ratio: Option<f64>,
		viewport_height: Option<f64>,
		origin: Option<Point3>,
		focal_length: Option<f64>,
	) -> Camera {
		let aspect_ratio = aspect_ratio.unwrap_or(16.0 / 9.0);
		let viewport_height = viewport_height.unwrap_or(2.0);
		let viewport_width = aspect_ratio * viewport_height;
		let focal_length = focal_length.unwrap_or(1.0);

		let origin = origin.unwrap_or(Point3::default());
		let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
		let vertical = Vec3::new(0.0, viewport_height, 0.0);
		let lower_left_corner = origin
			- horizontal / 2.0
			- vertical / 2.0
			- Vec3::new(0.0, 0.0, focal_length);

		Camera {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
		}
	}

	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray {
			origin: self.origin,
			direction: self.lower_left_corner
				+ u * self.horizontal
				+ v * self.vertical
				- self.origin,
		}
	}
}
