use crate::array::Array;
use crate::matrix::vector::Vector;

#[derive(Debug, Copy, Clone)]
pub struct Light {
	pub position: Vector<f32, 3>,
	pub diffuse: Array<f32, 3>,
	pub specular: Array<f32, 3>,
}
