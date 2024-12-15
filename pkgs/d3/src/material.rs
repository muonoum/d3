use array::Array;

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub emissive: Option<Array<f32, 3>>,
	pub ambient: Array<f32, 3>,
	pub diffuse: Array<f32, 3>,
	pub specular: Array<f32, 3>,
	pub shininess: f32,
}
