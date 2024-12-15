use array::Array;

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub ambient: Array<f32, 3>,
	pub diffuse: Array<f32, 3>,
	pub specular: Array<f32, 3>,
	pub shininess: f32,
}
