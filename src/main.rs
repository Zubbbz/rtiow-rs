use std::{env, path::Path};

use pbr::ProgressBar;
use rand::Rng;
use rtiow::{
	camera::Camera,
	hittable_list::HittableList,
	img::{push_pixel_color, save_image},
	material::{self, Material},
	ray_color,
	shapes::sphere::Sphere,
	utility::{random_f64, random_f64_range},
	vec::{Color, Point3, Vec3},
};

fn main() {
	// Image
	let aspect_ratio: f64 = 3.0 / 2.0;
	let img_width: u32 = 600;
	let img_height = (img_width as f64 / aspect_ratio) as u32;
	let spp: u32 = 500;
	let max_depth = 50;

	// World
	let world = random_scene();

	// Camera
	let lookfrom: Point3 = Point3::new(13.0, 2.0, 3.0);
	let lookat: Point3 = Point3::new(0.0, 0.0, 0.0);
	let vup: Vec3 = Vec3::new(0.0, 1.0, 0.0);
	let dist_to_focus: f64 = 10.0;
	let aperture: f64 = 0.1;

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

	let mut prog_bar = ProgressBar::new((img_height * img_width) as u64);
	println!("Rendering {} pixels at {} samples per pixel with a max ray depth of {}", img_height * img_width, spp, max_depth);
	// RENDER
	let mut rgb_buffer: Vec<u8> = vec![];
	for y in (0..img_height).rev() {
		for x in 0..img_width {
			prog_bar.inc();
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
	println!("Image written to {}",
		env::current_dir()
			.unwrap()
			.join(Path::new("image.png"))
			.display()
		);
	prog_bar.finish_print("Done!");
}

fn random_scene() -> Box<HittableList> {
	let mut world = Box::new(HittableList::new());

	let ground_material = Material {
		albedo: Color::new(0.5, 0.5, 0.5),
		surface: material::Surface::LambDiffuse,
		roughness: 0.0,
		ior: 0.0,
	};
	world.add(Box::new(Sphere {
		center: Point3::new(0.0, -1000.0, 0.0),
		radius: 1000.0,
		material: ground_material,
	}));

	for a in -11..11 {
		for b in -11..11 {
			let choose_mat = random_f64();
			let center: Point3 = Point3::new(
				a as f64 + 0.0 * random_f64(),
				0.2,
				b as f64 + 0.9 * random_f64(),
			);

			if ((center - Point3::new(4.0, 0.2, 0.0)).length()) > 0.9 {
				let sphere_material: Material;

				if choose_mat < 0.8 {
					// diffuse
					let albedo = Color::random() * Color::random();
					sphere_material = Material {
						albedo,
						surface: material::Surface::LambDiffuse,
						roughness: 0.0,
						ior: 0.0,
					};
					world.add(Box::new(Sphere {
						center,
						radius: 0.2,
						material: sphere_material,
					}));
				} else if choose_mat < 0.95 {
					// reflective
					let albedo = Color::random_range(0.5, 1.0);
					let fuzz = random_f64_range(0.0, 0.5);
					sphere_material = Material {
						albedo,
						surface: material::Surface::Reflective,
						roughness: fuzz,
						ior: 0.0,
					};
					world.add(Box::new(Sphere {
						center,
						radius: 0.2,
						material: sphere_material,
					}));
				} else {
					// glass
					let sphere_material = Material {
						albedo: Color::new(0.0, 0.0, 0.0),
						surface: material::Surface::Dielectric,
						roughness: 0.0,
						ior: 1.5,
					};
					world.add(Box::new(Sphere {
						center,
						radius: 0.2,
						material: sphere_material,
					}));
				}
			}
		}
	}

	let material1 = Material {
		albedo: Color::random(),
		surface: material::Surface::Dielectric,
		roughness: 0.0,
		ior: 1.5,
	};
	world.add(Box::new(Sphere {
		center: Point3::new(0.0, 1.0, 0.0),
		radius: 1.0,
		material: material1,
	}));

	let material2 = Material {
		albedo: Color::new(0.4, 0.2, 0.1),
		surface: material::Surface::LambDiffuse,
		roughness: 0.0,
		ior: 0.0,
	};
	world.add(Box::new(Sphere {
		center: Point3::new(-4.0, 1.0, 0.0),
		radius: 1.0,
		material: material2,
	}));

	let material3 = Material {
		albedo: Color::new(0.7, 0.6, 0.5),
		surface: material::Surface::Reflective,
		roughness: 0.0,
		ior: 0.0,
	};
	world.add(Box::new(Sphere {
		center: Point3::new(4.0, 1.0, 0.0),
		radius: 1.0,
		material: material3,
	}));

	world
}
