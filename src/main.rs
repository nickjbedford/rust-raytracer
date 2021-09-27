use crate::intersectable::Intersectable;

mod vector;
mod ray;
mod shapes;
mod intersectable;

fn main()
{
	test_reflect();
	test_ray();
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

fn test_ray()
{
	let ray = ray::new(
		vector::ZERO,
		vector::X);

	let sphere =
		shapes::sphere::new(
			vector::new(10.0, 0.0, 0.0),
			2.5);

	println!("Ray: {0}", ray);
	println!("Sphere: {0}", sphere);

	let intersection = sphere.intersect(&ray);
	match intersection {
		Some(i) => {
			let p0 = i.0;
			println!("Intersection (closest):  {0}", i.0);
			println!("Intersection (furthest): {0}", i.1);
		},
		_ => {}
	}
}