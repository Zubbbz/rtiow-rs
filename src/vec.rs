use core::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub struct Vec3 {
	pub e: [f64; 3],
}

impl Vec3 {
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Self { e: [x, y, z] }
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
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
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

impl Div<f64> for Vec3 {
	type Output = Self;

	fn div(self, t: f64) -> Self::Output {
		self * (1.0 / t)
	}
}

pub type Point3 = Vec3;
pub type Color = Vec3;
