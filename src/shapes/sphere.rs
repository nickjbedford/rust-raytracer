#![allow(dead_code)]

use super::super::intersectable::Intersectable;
use super::super::ray::Ray;
use super::super::ray;
use super::super::vector::Vec;
use super::super::vector::fp;

pub struct Sphere {
	pub center: Vec,
	pub radius: fp
}

pub fn new(center: Vec, radius: fp) -> Sphere {
	Sphere { center, radius }
}

impl Intersectable for Sphere {
	fn intersect(&self, ray: &Ray) -> Option<(Ray,Ray)> {
		let l = self.center - ray.origin;
		let tc = l.dot(ray.direction);
		if tc < 0.0 {
			return None;
		}
		let d = (tc * tc - l.len_sqr()).sqrt();
		if d > self.radius {
			return None;
		}
		let t1c = (self.radius * self.radius - d * d).sqrt();
		let p1 = *ray + (tc - t1c);
		let p2 = *ray + (tc + t1c);
		let n1 = (p1 - self.center).unit();
		let n2 = (p2 - self.center).unit();
		return Some((ray::new(p1, n1), ray::new(p2, n2)));
	}
}

impl std::fmt::Display for Sphere {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{0} ({1})", self.center, self.radius);
	}
}