mod vector;

fn main()
{
	test_reflect();
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