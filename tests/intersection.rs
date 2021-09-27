use intersection;
use ray;
use vector;

#[test]
fn ray_away_from_sphere_has_no_intersection() {
    let ray = ray::new(vector::ZERO, vector::X);
}