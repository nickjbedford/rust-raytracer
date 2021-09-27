#![allow(dead_code)]

pub use super::ray::Ray;
pub use std::option::Option;

pub trait Intersectable {
	fn intersect(&self, ray: &Ray) -> Option<(Ray,Ray)>;
}