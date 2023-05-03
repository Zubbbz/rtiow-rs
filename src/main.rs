use rand::Rng;
use rtiow::{
	camera::Camera,
	hittable_list::HittableList,
	img::{push_pixel_color, save_image},
	ray_color,
	shapes::sphere::Sphere,
	vec::{Color, Vec3},
};

fn main() {
	// Image
	let aspect_ratio: f64 = 16.0 / 9.0;
	let img_width: u32 = 1280;
	let img_height = (img_width as f64 / aspect_ratio) as u32;
	let spp: u32 = 20;

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

	// Camera
	let camera: Camera = Camera::new(Some(aspect_ratio), None, None, None);

	// SETUP RNG
	let mut rng = rand::thread_rng();

	// RENDER
	let mut rgb_buffer: Vec<u8> = vec![];
	for y in (0..img_height).rev() {
		for x in 0..img_width {
			let mut pixel_color: Color = Color::default();
			for _s in 0..spp {
				let u: f64 =
					(x as f64 + rng.gen::<f64>()) / (img_width as f64 - 1.0);
				let v: f64 =
					(y as f64 + rng.gen::<f64>()) / (img_height as f64 - 1.0);
				let ray = camera.get_ray(u, v);
				pixel_color = pixel_color + ray_color(None, &ray, &world);
			}
			push_pixel_color(&mut rgb_buffer, &mut pixel_color, None, spp);
		}
	}

	save_image(None, (img_width, img_height), &rgb_buffer).unwrap();
	println!("\nDone.\n");
}
