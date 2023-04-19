use img::lerp_colors;
use ray::Ray;
use vec::{dot, Color, Point3, Vec3};

pub mod img;
pub mod ray;
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

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
	let oc: Vec3 = ray.origin - *center;
	let a = ray.direction.length_squared();
	let half_b = dot(&oc, &ray.direction);
	let c = oc.length_squared() - radius * radius;

	let descriminant = half_b * half_b - a * c;
	if descriminant < 0.0 {
		-1.0
	} else {
		(-half_b - descriminant.sqrt()) / a
	}
}

pub fn ray_color(ray: &Ray, bg_colors: Option<(Color, Color)>) -> Color {
	let mut t = hit_sphere(
		&Point3 {
			e: [0.0, 0.0, -1.0],
		},
		0.5,
		ray,
	);
	if t > 0.0 {
		let n = (ray.at(t)
			- Vec3 {
				e: [0.0, 0.0, -1.0],
			})
		.normalize();
		return 0.5
			* Color {
				e: [n.x() + 1.0, n.y() + 1.0, n.z() + 1.0],
			};
	}
	let normalized_dir = ray.direction.normalize();
	t = 0.5 * (normalized_dir.y() + 1.0);

	let colors = bg_colors.unwrap_or((
		Color { e: [1.0, 1.0, 1.0] },
		Color { e: [0.5, 0.7, 1.0] },
	));
	lerp_colors(t, colors)
}
