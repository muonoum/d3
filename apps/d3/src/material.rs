use array::array;
use array::Array;

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub emissive_color: Array<f32, 3>,
	pub ambient_reflection: Array<f32, 3>,
	pub diffuse_reflection: Array<f32, 3>,
	pub specular_reflection: Array<f32, 3>,
	pub specular_exponent: i32,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			emissive_color: array![0.0; 3],
			ambient_reflection: array![1.0; 3],
			diffuse_reflection: array![0.0; 3],
			specular_reflection: array![0.0; 3],
			specular_exponent: 1,
		}
	}
}
