use crate::vec::{Point3, Vec3};

pub struct Ray {
	pub origin: Point3,
	pub direction: Vec3,
}

impl Ray {
	pub fn new(origin: Point3, dir: Vec3) -> Self {
		Ray {
			origin: origin,
			direction: dir,
		}
	}

	pub fn at(&self, t: f64) -> Point3 {
		self.origin + self.direction * t
	}
}
