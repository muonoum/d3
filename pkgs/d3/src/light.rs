use array::Array;
use matrix::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Light {
	pub position: Vector<f32, 3>,
	pub diffuse: Array<f32, 3>,
	pub specular: Array<f32, 3>,
}
