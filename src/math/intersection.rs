#![allow(dead_code)]

use super::ray;
use super::vector::Vec;
use ray::Ray;

pub enum Location {
    Inside,
    Outside
}

pub struct Intersection {
    pub ray: Ray,
    pub location: Location
}

pub fn inside(origin: Vec, direction: Vec) -> Intersection {
    Intersection { ray: ray::new(origin, direction), location: Location::Inside }
}

pub fn outside(origin: Vec, direction: Vec) -> Intersection {
    Intersection { ray: ray::new(origin, direction), location: Location::Outside }
}

pub trait Intersectable {
	fn intersect(&self, ray: &Ray) -> Option<Ray>;
}