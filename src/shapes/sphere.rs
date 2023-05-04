use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{dot, Point3, Vec3};

#[derive(Clone, Copy, Default)]
pub struct Sphere {
	pub center: Point3,
	pub radius: f64,
	pub material: Material,
}

impl Sphere {
	pub fn new(center: Point3, radius: f64, material: Material) -> Self {
		Self {
			center,
			radius,
			material,
		}
	}
}

impl Hittable for Sphere {
	fn hit(
		&self,
		ray: &Ray,
		t_min_max: (f64, f64),
		rec: &mut HitRecord,
	) -> bool {
		let oc: Vec3 = ray.origin - self.center;
		let a = ray.direction.length_squared();
		let half_b = dot(&oc, &ray.direction);
		let c = oc.length_squared() - self.radius * self.radius;

		let discriminant = half_b * half_b - a * c;
		if discriminant < 0.0 {
			return false;
		}
		let sqrtd = num::Float::sqrt(discriminant);

		// Find the nearest root that lies in the acceptable range.
		let mut root = (-half_b - sqrtd) / a;
		if root < t_min_max.0 || t_min_max.1 < root {
			root = (-half_b + sqrtd) / a;
			if root < t_min_max.0 || t_min_max.1 < root {
				return false;
			}
		}

		rec.t = root;
		rec.intersection = ray.at(rec.t);
		let outward_normal: Vec3 =
			(rec.intersection - self.center) / self.radius;
		rec.set_face_normal(ray, &outward_normal);
		rec.material = self.material;

		true
	}
}
