use array::Array;
use matrix::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Light {
	pub position: Vector<f32, 3>,
	pub ambient_color: Array<f32, 3>,
	pub diffuse_color: Array<f32, 3>,
	pub specular_color: Array<f32, 3>,
}