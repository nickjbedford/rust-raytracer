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
use intersection::Location;
use vector::Vec;

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

trait IsApproximately<T> {
    fn is_approximately(&self, value: T) -> bool;
}

impl IsApproximately<fp> for fp {
    fn is_approximately(&self, value: fp) -> bool {
        let min = value - EPSILON;
        let max = value + EPSILON;
        return *self >= min && *self <= max;
    }
}

impl IsApproximately<Vec> for Vec {
    fn is_approximately(&self, value: Vec) -> bool {
        self.x.is_approximately(value.x) &&
            self.y.is_approximately(value.y) &&
            self.z.is_approximately(value.z)
    }
}

#[test]
pub fn ray_away_from_sphere_has_no_intersection()
{
    let ray = ray::new(vector::ZERO, vector::X);
    let sphere = sphere::new(-vector::X, 0.5);

    assert!(sphere.intersect(&ray).is_none());
}

#[test]
pub fn ray_toward_sphere_has_intersection()
{
    let ray = ray::new(vector::ZERO, vector::X);
    let sphere = sphere::new(vector::X * 5.0, 0.5);
    let intersection = sphere.intersect(&ray);

    assert!(intersection.is_some());
    match intersection {
        Some(i) => {
            match i.location {
                Location::Outside => assert!(true),
                Location::Inside => assert!(false)
            };
            assert!(i.ray.origin.is_approximately(vector::new(4.5, 0.0, 0.0)));
            assert!(i.ray.direction.is_approximately(vector::new(-1.0, 0.0, 0.0)));
        },
        _ => {}
    }
}