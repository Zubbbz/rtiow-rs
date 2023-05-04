use crate::{
	hittable::HitRecord,
	ray::Ray,
	utility::random_f64,
	vec::{dot, Color, Vec3},
};

#[derive(Clone, Copy, Default)]
pub enum Surface {
	#[default]
	LambDiffuse,
	Reflective,
	Dielectric,
}

#[derive(Clone, Copy, Default)]
pub struct Material {
	pub albedo: Color,
	pub surface: Surface,
	pub roughness: f64,
	pub ior: f64,
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

			Surface::Dielectric => {
				*attenuation = Color::new(1.0, 1.0, 1.0);
				let refraction_ratio: f64 = if rec.front_face {
					1.0 / self.ior
				} else {
					self.ior
				};

				let normal: Vec3 = ray_in.direction.normalize();
				let cos_theta: f64 = dot(&-normal, &rec.normal).min(1.0);
				let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

				let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;

				let direction: Vec3 = if cannot_refract
					|| Material::reflectance(cos_theta, refraction_ratio)
						> random_f64()
				{
					Vec3::reflect(&normal, &rec.normal)
				} else {
					Vec3::refract(&normal, &rec.normal, refraction_ratio)
				};

				*scattered = Ray::new(rec.intersection, direction);
				true
			}
		}
	}

	pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
		// Schlick's approximation for reflectance
		let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
		r0 = r0 * r0;
		r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
	}
}
