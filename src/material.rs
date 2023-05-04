use crate::{
	hittable::HitRecord,
	ray::Ray,
	vec::{dot, Color, Vec3},
};

#[derive(Clone, Copy, Default, Debug)]
pub enum Surface {
	#[default]
	LambDiffuse,
	Reflective,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Material {
	pub albedo: Color,
	pub surface: Surface,
	pub roughness: f64,
}

impl Material {
	pub fn sample(
		&self,
		ray_in: &Ray,
		rec: &HitRecord,
		attenuation: &mut Color,
		scattered: &mut Ray,
	) -> bool {
		match self.surface {
			Surface::LambDiffuse => {
				let mut scatter_direction = rec.normal + Vec3::random_normal();
				if scatter_direction.near_zero() {
					scatter_direction = rec.normal;
				}

				*scattered = Ray::new(rec.intersection, scatter_direction);
				*attenuation = self.albedo;
				true
			}

			Surface::Reflective => {
				let reflected =
					Vec3::reflect(&ray_in.direction.normalize(), &rec.normal);
				*scattered = Ray::new(
					rec.intersection,
					reflected + self.roughness * Vec3::rand_in_unit_sphere(),
				);
				*attenuation = self.albedo;

				dot(&scattered.direction, &rec.normal) > 0.0
			}
		}
	}
}
