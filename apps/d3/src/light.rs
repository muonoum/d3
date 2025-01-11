use array::Array;
use matrix::Vector;

pub struct Light {
	pub position: Vector<f32, 3>,
	pub diffuse_color: Array<f32, 3>,
	pub specular_color: Array<f32, 3>,
}

pub fn blinn_phong(
	position: Vector<f32, 3>,
	normal: Vector<f32, 3>,
	uv: Option<Vector<f32, 2>>,
	camera: Vector<f32, 3>,
	lights: &[Light],
	material: &obj::Material,
) -> Array<f32, 3> {
	let camera_dir = (camera - position).normalize();

	let color = (lights.iter()).fold(material.emissive(uv), |sum, light| {
		let light_dir = (light.position - position).normalize();

		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let halfway_vector = (light_dir + camera_dir).normalize();
		let specular = normal
			.dot(halfway_vector)
			.powi(material.specular_exponent(uv) as i32);

		sum + material.diffuse(uv) * diffuse * light.diffuse_color
			+ material.specular(uv) * specular * light.specular_color
	});

	(color * 255.0).clamp(0.0, 255.0)
}
