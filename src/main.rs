mod shapes;
mod math;

use math::vector;
use math::ray;
use math::fp;

fn main()
{
	test_reflect();
	test_ray(0.0);
}

fn test_reflect()
{
	let incident = vector::new(1.0, -0.25, 0.0).unit();
	let normal = -vector::Y;
	let reflected = incident.reflect(normal);

	println!("Incident: {0}", incident);
	println!("Normal: {0}", normal);
	println!("Reflected: {0}", reflected);
}

fn test_ray(y: fp)
{
	println!("===================");
	println!("SPHERE INTERSECTION");
	println!("===================");

	let ray = ray::new(
		vector::ZERO,
		vector::X);

	let sphere =
		shapes::sphere::new(
			vector::new(10.0, y, 0.0),
			2.5);

	println!("Ray: {0}", ray);
	println!("Sphere: {0}", sphere);

	match sphere.intersect(&ray) {
		Some(i) => {
			println!("Intersection:  {0}", i);
		},
		None => {
			println!("No intersection!")
		}
	}
}