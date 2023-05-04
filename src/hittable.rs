use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{dot, Point3, Vec3};

#[derive(Clone, Copy, Default, Debug)]
pub struct HitRecord {
	pub intersection: Point3,
	pub normal: Vec3,
	pub material: Material,
	pub t: f64,
	pub front_face: bool,
}

impl HitRecord {
	pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
		self.front_face = dot(&ray.direction, outward_normal) < 0.0;
		self.normal = if self.front_face {
			*outward_normal
		} else {
			-*outward_normal
		};
	}
}

pub trait Hittable {
	fn hit(
		&self,
		ray: &Ray,
		t_min_max: (f64, f64),
		rec: &mut HitRecord,
	) -> bool;
}
