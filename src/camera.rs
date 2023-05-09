use crate::{
	ray::Ray,
	vec::{cross, Point3, Vec3},
};

pub struct Camera {
	pub origin: Point3,
	pub lower_left_corner: Point3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub lens_radius: f64,
	pub u: Vec3,
	pub v: Vec3,
	pub w: Vec3,
}

impl Camera {
	pub fn new(
		lookfrom: Point3,
		lookat: Point3,
		vup: Vec3,
		vfov: f64,
		aspect_ratio: f64,
		aperture: f64,
		focus_dist: f64,
	) -> Camera {
		let theta = vfov.to_radians();
		let h = (theta / 2.0).tan();
		let viewport_height = 2.0 * h;
		let viewport_width = aspect_ratio * viewport_height;

		let w = (lookfrom - lookat).normalize();
		let u = cross(&vup, &w).normalize();
		let v = cross(&w, &u);

		let origin = lookfrom;
		let horizontal = focus_dist * viewport_width * u;
		let vertical = focus_dist * viewport_height * v;
		let lower_left_corner =
			origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

		let lens_radius = aperture / 2.0;

		Camera {
			origin,
			lower_left_corner,
			horizontal,
			vertical,
			lens_radius,
			u,
			v,
			w,
		}
	}

	pub fn get_ray(&self, s: f64, t: f64) -> Ray {
		let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
		let offset: Vec3 = self.u * rd.x() + self.v * rd.y();

		Ray::new(
			self.origin + offset,
			self.lower_left_corner + s * self.horizontal + t * self.vertical
				- self.origin - offset,
		)
	}
}
