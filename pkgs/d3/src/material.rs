use array::Array;

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub emissive_color: Array<f32, 3>,
	pub ambient_reflection: Array<f32, 3>,
	pub diffuse_reflection: Array<f32, 3>,
	pub specular_reflection: Array<f32, 3>,
	pub specular_exponent: f32,
}
