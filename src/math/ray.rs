#![allow(dead_code)]

pub use super::vector::Vec;
pub use super::fp;
use std::ops;

#[derive(Copy)]
pub struct Ray {
	pub origin: Vec,
	pub direction: Vec
}

pub fn new(origin: Vec, direction: Vec) -> Ray {
	Ray { origin, direction }
}

impl Clone for Ray {
	fn clone(&self) -> Self {
		return new(self.origin, self.direction)
	}
}

impl Ray {
	fn project(self, distance: fp) -> Vec {
		self.origin + (self.direction * distance)
	}
}

impl ops::Add<fp> for Ray {
	type Output = Vec;
	fn add(self, distance: fp) -> Vec {
		self.project(distance)
	}
}

impl std::fmt::Display for Ray {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{0} -> {1}", self.origin, self.direction);
	}
}