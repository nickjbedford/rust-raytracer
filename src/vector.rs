#![allow(dead_code)]

use std::ops;

#[allow(non_camel_case_types)]
pub type fp = f32;

#[derive(Copy)]
pub struct Vec {
	pub x: fp,
	pub y: fp,
	pub z: fp
}

pub const ZERO: Vec = Vec { x: 0.0, y: 0.0, z: 0.0 };
pub const X: Vec = Vec { x: 1.0, y: 0.0, z: 0.0 };
pub const Y: Vec = Vec { x: 0.0, y: 1.0, z: 0.0 };
pub const Z: Vec = Vec { x: 0.0, y: 0.0, z: 1.0 };

pub fn new(x: fp, y: fp, z: fp) -> Vec {
	Vec { x, y, z }
}

impl Vec {
	pub fn len(self) -> fp {
		return self.dot(self).sqrt();
	}

	pub fn dot(self, rhs: Vec) -> fp {
		return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
	}

	pub fn unit(self) -> Vec {
		let len = self.len();
		if len == 0.0 {
			return ZERO;
		}
		return self / len;
	}

	pub fn reflect(self, normal: Vec) -> Vec {
		return self - (self.dot(normal) * 2.0) * normal
	}
}

impl Clone for Vec {
	fn clone(&self) -> Self {
		return new(self.x, self.y, self.z)
	}
}

impl ops::Neg for Vec {
	type Output = Vec;
	fn neg(self) -> Vec {
		return new(-self.x, -self.y, -self.z);
	}
}

impl ops::Add<Vec> for Vec {
	type Output = Vec;
	fn add(self, rhs: Vec) -> Vec {
		new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
	}
}

impl ops::Sub<Vec> for Vec {
	type Output = Vec;
	fn sub(self, rhs: Vec) -> Vec {
		new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
	}
}

impl ops::Mul<fp> for Vec {
	type Output = Vec;
	fn mul(self, rhs: fp) -> Vec {
		new(self.x * rhs, self.y * rhs, self.z * rhs)
	}
}

impl ops::Mul<Vec> for fp {
	type Output = Vec;
	fn mul(self, rhs: Vec) -> Vec {
		rhs * self
	}
}

impl ops::Div<fp> for Vec {
	type Output = Vec;
	fn div(self, rhs: fp) -> Vec {
		new(self.x / rhs, self.y / rhs, self.z / rhs)
	}
}

impl ops::Div<Vec> for fp {
	type Output = Vec;
	fn div(self, rhs: Vec) -> Vec {
		rhs / self
	}
}

impl std::fmt::Display for Vec {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{{{0}, {1}, {2}}}", self.x, self.y, self.z);
	}
}