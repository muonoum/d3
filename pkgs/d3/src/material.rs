use array::Array;

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub ambient: Array<f32, 3>,
	pub diffuse: f32,
	pub shininess: f32,
	pub specular: f32,
}
