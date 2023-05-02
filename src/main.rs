use rtiow::{
	hittable_list::HittableList,
	img::{push_pixel_color, save_image},
	ray::Ray,
	ray_color,
	shapes::sphere::Sphere,
	vec::{Point3, Vec3},
};

fn main() {
	// World
	let mut world = Box::new(HittableList::new());

	world.add(Box::new(Sphere {
		center: Vec3 {
			e: [0.0, 0.0, -1.0],
		},
		radius: 0.5,
	}));
	world.add(Box::new(Sphere {
		center: Vec3 {
			e: [0.0, -100.5, -1.0],
		},
		radius: 100.0,
	}));

	// Const variables
	// Image
	let aspect_ratio: f64 = 16.0 / 9.0;
	let img_width: u32 = 1920;
	let img_height = (img_width as f64 / aspect_ratio) as u32;

	// Camera
	let viewport_height: f64 = 2.0;
	let viewport_width: f64 = aspect_ratio * viewport_height;
	let focal_length: f64 = 1.0;

	let cam_origin = Point3 { e: [0.0, 0.0, 0.0] };
	let horizontal = Vec3 {
		e: [viewport_width, 0.0, 0.0],
	};
	let vertical = Vec3 {
		e: [0.0, viewport_height, 0.0],
	};
	let lower_left_corner = cam_origin
		- horizontal / 2.0
		- vertical / 2.0
		- Vec3 {
			e: [0.0, 0.0, focal_length],
		};
	// END VARIABLES

	// RENDER
	let mut rgb_buffer: Vec<u8> = vec![];
	for y in (0..img_height).rev() {
		for x in 0..img_width {
			let u = x as f64 / (img_width - 1) as f64;
			let v = y as f64 / (img_height - 1) as f64;

			let ray = Ray {
				origin: cam_origin,
				direction: lower_left_corner
					+ (u * horizontal) + (v * vertical)
					- cam_origin,
			};

			let pixel_color = ray_color(None, &ray, &world);
			push_pixel_color(&mut rgb_buffer, pixel_color, None);
		}
	}

	save_image(None, (img_width, img_height), &rgb_buffer).unwrap();
	println!("\nDone.\n");
}
