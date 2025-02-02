use array::{Array, array};
use matrix::Vector;

#[derive(Copy, Clone)]
pub struct Light {
	pub diffuse_color: Array<f32, 3>,
	pub specular_color: Array<f32, 3>,
	pub position: Vector<f32, 3>,
}

#[inline]
pub fn blinn_phong(
	current: Array<f32, 3>,
	position: Vector<f32, 3>,
	normal: Vector<f32, 3>,
	uv: Option<Vector<f32, 2>>,
	camera: Vector<f32, 3>,
	lights: &[Light],
	material: &obj::Material,
) -> Option<Array<f32, 3>> {
	// TODO alpha
	let alpha = material.alpha(uv);
	if alpha == array![0.0; 3] {
		return None;
	}

	let camera_dir = (camera - position).normalize();
	let diffuse_reflection = material.diffuse(uv);
	let specular_reflection = material.specular(uv);
	let specular_exponent = material.specular_exponent(uv);

	let color = (lights.iter()).fold(material.emissive(uv), |sum, light| {
		let light_dir = (light.position - position).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let halfway_vector = (light_dir + camera_dir).normalize();
		let specular = normal.dot(halfway_vector).powi(specular_exponent as i32);

		sum + diffuse_reflection * diffuse * light.diffuse_color
			+ specular_reflection * specular * light.specular_color
	});

	let color = color * alpha + current * (array![1.0; 3] - alpha);
	Some(color.clamp(0.0, 1.0))
}
