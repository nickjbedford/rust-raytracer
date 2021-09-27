#![allow(dead_code)]

use super::super::math;

use math::intersection::Intersectable;
use math::vector::Vec;
use math::ray::Ray;
use math::fp;

pub struct Sphere {
	pub center: Vec,
	pub radius: fp
}

pub fn new(center: Vec, radius: fp) -> Sphere {
	Sphere { center, radius }
}

impl Intersectable for Sphere {
	fn intersect(&self, ray: &Ray) -> Option<Ray> {
		let ray_to_center = self.center - ray.origin;
		let ray_to_center_dist = ray_to_center.len();
		
		if ray_to_center_dist >= self.radius { // ray is outside or on surface of sphere
			if ray_to_center.dot(ray.direction) < 0.0 { // ray is pointing away from sphere
				return None
			}
		}

		return None
	}
}

impl std::fmt::Display for Sphere {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{0} ({1})", self.center, self.radius);
	}
}