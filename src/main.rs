use std::f64::consts::PI;

use rand::Rng;
use rtiow::{
	camera::Camera,
	hittable_list::HittableList,
	img::{push_pixel_color, save_image},
	material::{self, Material},
	ray_color,
	shapes::sphere::Sphere,
	vec::{Color, Point3, Vec3},
};

fn main() {
	// Image
	let aspect_ratio: f64 = 16.0 / 9.0;
	let img_width: u32 = 1280;
	let img_height = (img_width as f64 / aspect_ratio) as u32;
	let spp: u32 = 100;
	let max_depth = 50;

	// World
	let _r = (PI / 4.0).cos();
	let mut world = Box::new(HittableList::new());

	let material_ground = Material {
		albedo: Color::new(0.8, 0.8, 0.0),
		surface: material::Surface::LambDiffuse,
		roughness: 0.0,
		ior: 0.0,
	};
	let material_center = Material {
		albedo: Color::new(0.1, 0.2, 0.5),
		surface: material::Surface::LambDiffuse,
		roughness: 0.0,
		ior: 1.333,
	};
	let material_left = Material {
		albedo: Color::default(),
		surface: material::Surface::Dielectric,
		roughness: 0.0,
		ior: 1.50,
	};
	let material_right = Material {
		albedo: Color::new(0.8, 0.6, 0.2),
		surface: material::Surface::Reflective,
		roughness: 0.0,
		ior: 0.0,
	};

	world.add(Box::new(Sphere {
		center: Point3::new(0.0, -100.5, -1.0),
		radius: 100.0,
		material: material_ground,
	}));

	world.add(Box::new(Sphere {
		center: Point3::new(0.0, 0.0, -1.0),
		radius: 0.5,
		material: material_center,
	}));
	world.add(Box::new(Sphere {
		center: Point3::new(-1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_left,
	}));
	world.add(Box::new(Sphere {
		center: Point3::new(-1.0, 0.0, -1.0),
		radius: -0.45,
		material: material_left,
	}));
	world.add(Box::new(Sphere {
		center: Point3::new(1.0, 0.0, -1.0),
		radius: 0.5,
		material: material_right,
	}));

	// Camera
	let lookfrom: Point3 = Point3::new(3.0, 3.0, 2.0);
	let lookat: Point3 = Point3::new(0.0, 0.0, -1.0);
	let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
	let dist_to_focus: f64 = (lookfrom - lookat).length();
	let aperture: f64 = 2.0;

	let camera: Camera = Camera::new(
		lookfrom,
		lookat,
		vup,
		20.0,
		aspect_ratio,
		aperture,
		dist_to_focus,
	);

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
				pixel_color =
					pixel_color + ray_color(None, &ray, &world, max_depth);
			}
			push_pixel_color(&mut rgb_buffer, &mut pixel_color, None, spp);
		}
	}

	save_image(None, (img_width, img_height), &rgb_buffer).unwrap();
	println!("\nDone.\n");
}
