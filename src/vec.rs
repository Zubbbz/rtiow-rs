use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::Rng;

use crate::utility::{random_f64, random_f64_range};

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
	pub e: [f64; 3],
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self { e: [x, y, z] }
	}

	pub fn new_rand(minmax: Option<(f64, f64)>) -> Self {
		let mut rng = rand::thread_rng();
		let minmax = minmax.unwrap_or((0.0, 1.0));

		Self {
			e: [
				rng.gen_range(minmax.0..minmax.1),
				rng.gen_range(minmax.0..minmax.1),
				rng.gen_range(minmax.0..minmax.1),
			],
		}
	}
	pub fn random_normal() -> Self {
		Vec3::rand_in_unit_sphere().normalize()
	}

	pub fn rand_in_unit_sphere() -> Self {
		loop {
			let p = Vec3::new_rand(Some((-1.0, 1.0)));
			if p.length_squared() >= 1.0 {
				continue;
			}
			return p;
		}
	}

	pub fn random_in_hemisphere(normal: Vec3) -> Self {
		let in_unit_sphere = Vec3::rand_in_unit_sphere();
		if dot(&in_unit_sphere, &normal) > 0.0 {
			// In the same hemisphere as the normal
			in_unit_sphere
		} else {
			-in_unit_sphere
		}
	}

	pub fn random_in_unit_disk() -> Self {
		loop {
			let p = Vec3::new(
				random_f64_range(-1.0, 1.0),
				random_f64_range(-1.0, 1.0),
				0.0,
			);
			if p.length_squared() >= 1.0 {
				continue;
			}
			return p;
		}
	}

	pub fn random() -> Self {
		Vec3::new(random_f64(), random_f64(), random_f64())
	}

	pub fn random_range(min: f64, max: f64) -> Self {
		Vec3::new(
			random_f64_range(min, max),
			random_f64_range(min, max),
			random_f64_range(min, max),
		)
	}

	pub fn x(&self) -> f64 {
		self.e[0]
	}

	pub fn y(&self) -> f64 {
		self.e[1]
	}

	pub fn z(&self) -> f64 {
		self.e[2]
	}

	pub fn length_squared(&self) -> f64 {
		self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
	}

	pub fn length(&self) -> f64 {
		self.length_squared().sqrt()
	}

	pub fn normalize(&self) -> Self {
		*self / self.length()
	}

	pub fn near_zero(&self) -> bool {
		let s = 1e-8;
		f64::abs(self.e[0]) < s
			&& f64::abs(self.e[1]) < s
			&& f64::abs(self.e[2]) < s
	}

	pub fn reflect(vec: &Vec3, nml: &Vec3) -> Self {
		*vec - 2.0 * dot(vec, nml) * *nml
	}

	pub fn refract(uv: &Vec3, nml: &Vec3, etai_over_etat: f64) -> Self {
		let cos_theta = dot(&-*uv, nml).min(1.0);
		let r_out_perp: Vec3 = etai_over_etat * (*uv + cos_theta * *nml);
		let r_out_parallel: Vec3 =
			-(1.0 - r_out_perp.length_squared()).abs().sqrt() * *nml;
		r_out_perp + r_out_parallel
	}

	pub fn cross(&self, v: &Vec3) -> Self {
		Vec3::new(
			self.e[1] * v.e[2] - self.e[2] * v.e[1],
			self.e[2] * v.e[0] - self.e[0] * v.e[2],
			self.e[0] * v.e[1] - self.e[1] * v.e[0],
		)
	}

	pub fn dot(&self, v: &Vec3) -> f64 {
		self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
	}
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "x: {}, y: {}, Z: {}", self.e[0], self.e[1], self.e[2])
	}
}

impl Neg for Vec3 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::new(-self.e[0], -self.e[1], -self.e[2])
	}
}

impl Add for Vec3 {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self::new(
			self.e[0] + other.e[0],
			self.e[1] + other.e[1],
			self.e[2] + other.e[2],
		)
	}
}

impl Sub for Vec3 {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self::new(
			self.e[0] - other.e[0],
			self.e[1] - other.e[1],
			self.e[2] - other.e[2],
		)
	}
}

impl Mul<f64> for Vec3 {
	type Output = Self;

	fn mul(self, t: f64) -> Self::Output {
		Self::new(self.e[0] * t, self.e[1] * t, self.e[2] * t)
	}
}

impl Mul<Vec3> for f64 {
	type Output = Vec3;

	fn mul(self, v: Vec3) -> Self::Output {
		v * self
	}
}

impl Mul<Vec3> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Self::Output {
		Vec3 {
			e: [
				self.e[0] * other.e[0],
				self.e[1] * other.e[1],
				self.e[2] * other.e[2],
			],
		}
	}
}

impl Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, t: f64) -> Self::Output {
		self * (1.0 / t)
	}
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
	Vec3::new(
		u.e[1] * v.e[2] - u.e[2] * v.e[1],
		u.e[2] * v.e[0] - u.e[0] * v.e[2],
		u.e[0] * v.e[1] - u.e[1] * v.e[0],
	)
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
	u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub type Point3 = Vec3;
pub type Color = Vec3;
