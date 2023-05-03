use std::f64::INFINITY;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use img::lerp_colors;
use ray::Ray;
use vec::{Color, Point3, Vec3};

pub mod camera;
pub mod hittable;
pub mod hittable_list;
pub mod img;
pub mod ray;
pub mod shapes;
pub mod utility;
pub mod vec;

#[cfg(test)]
mod tests {
	use crate::vec::{dot, Vec3};

	const A: Vec3 = Vec3 {
		e: [33.659, 2.582, 12.811],
	};
	const B: Vec3 = Vec3 {
		e: [27.125, 3.58, 32.53],
	};

	#[test]
	fn test_dot_product() {
		let dot = dot(&A, &B);
		assert_eq!(dot, 1338.985765);
		println!("{dot}");
	}
}

pub fn ray_color(
	bg_colors: Option<(Color, Color)>,
	ray: &Ray,
	world: &HittableList,
	depth: u32,
) -> Color {
	let mut rec = HitRecord::default();

	if depth == 0 {
		return Color::default();
	}

	if world.hit(ray, (0.001, INFINITY), &mut rec) {
		let target: Point3 =
			rec.intersection + rec.normal + Vec3::new_rand_in_sphere();
		return 0.5
			* ray_color(
				None,
				&Ray {
					origin: rec.intersection,
					direction: target - rec.intersection,
				},
				world,
				depth - 1,
			);
	}

	let normalized = ray.direction.normalize();
	let t = 0.5 * (normalized.y() + 1.0);

	let colors = bg_colors.unwrap_or((
		Color { e: [1.0, 1.0, 1.0] },
		Color { e: [0.5, 0.7, 1.0] },
	));

	lerp_colors(t, colors)
}
