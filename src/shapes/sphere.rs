#![allow(dead_code)]

use super::super::math;

use math::intersection;
use intersection::Intersectable;
use intersection::Intersection;
use math::vector::Vec;
use math::ray;
use ray::Ray;
use math::fp;

pub struct Sphere {
	pub center: Vec,
	pub radius: fp
}

pub fn new(center: Vec, radius: fp) -> Sphere {
	Sphere { center, radius }
}

impl Intersectable for Sphere {
	fn intersect(&self, ray: &Ray) -> Option<Intersection> {
		let origin = ray.origin;
		let dir = ray.direction;
		let center = self.center;
		let radius = self.radius;
		let radius_sqr = radius * radius;
		
		let to_center = center - origin;
		let dist_to_center_sqr = to_center.len_sqr();
		
		// ray is outside or on surface of sphere
		if dist_to_center_sqr >= radius_sqr {

			let dot = to_center.dot(dir);

			// ray is pointing away from sphere
			if dot < 0.0 {
				return None
			}
			
			// determine point
			let closest_approach = to_center - (dir * dot);
			let distance_sqr = closest_approach.len_sqr();

			if distance_sqr > radius_sqr {
				return None
			}

			let h = (radius_sqr - distance_sqr).sqrt();
			let i = closest_approach - (dir * h);
			let intersection = center + i;
			let normal = i / radius;

			return Some(intersection::outside(intersection, normal))
		}

		return None
	}
}

impl std::fmt::Display for Sphere {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		return write!(f, "{0} ({1})", self.center, self.radius);
	}
}