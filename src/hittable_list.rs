use crate::{
	hittable::{HitRecord, Hittable},
	ray::Ray,
};

pub struct HittableList {
	pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
	pub fn new() -> Self {
		Self {
			objects: Vec::new(),
		}
	}

	pub fn clear(&mut self) {
		self.objects.clear();
	}

	pub fn add(&mut self, object: Box<dyn Hittable>) {
		self.objects.push(object);
	}
}

impl Default for HittableList {
	fn default() -> Self {
		Self::new()
	}
}

impl Hittable for HittableList {
	fn hit(
		&self,
		ray: &Ray,
		t_min_max: (f64, f64),
		rec: &mut HitRecord,
	) -> bool {
		let mut _temp_rec = HitRecord::default();
		let mut hit_anything = false;
		let mut closest_so_far = t_min_max.1;

		for object in &self.objects {
			if object.hit(ray, (t_min_max.0, closest_so_far), rec) {
				hit_anything = true;
				closest_so_far = rec.t;
				_temp_rec = *rec;
			}
		}
		hit_anything
	}
}
