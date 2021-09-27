#[allow(unused_imports)]

use crate::math;
use crate::shapes;

use std::option::Option;
use math::intersection;
use math::fp;
use math::ray;
use math::vector;
use shapes::sphere;
use intersection::Intersectable;

trait OptionIs {
    fn is_some(&self) -> bool;
    fn is_none(&self) -> bool;
}

impl<T> OptionIs for Option<T> {
    fn is_some(&self) -> bool {
        match *self {
            Some(_) => true,
            None => false
        }
    }

    fn is_none(&self) -> bool {
        match *self {
            Some(_) => false,
            None => true
        }
    }
}

const EPSILON: fp = 0.0001;

trait IsApproximately {
    fn is_approximately(&self) -> bool;
}

impl IsApproximately for fp {
    fn is_approximately(&self) -> bool {
        let min = self - EPSILON;
        let max = self + EPSILON;
        return *self >= min && *self <= max;
    }
}

#[test]
fn ray_away_from_sphere_has_no_intersection()
{
    let ray = ray::new(vector::ZERO, vector::X);
    let sphere = sphere::new(-vector::X, 0.5);

    assert!(sphere.intersect(&ray).is_none());
}

#[test]
fn ray_toward_sphere_has_intersection()
{
    let ray = ray::new(vector::ZERO, vector::X);
    let sphere = sphere::new(vector::X * 5.0, 0.5);
    let intersection = sphere.intersect(&ray);

    assert!(intersection.is_some());
}