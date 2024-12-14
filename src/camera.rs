use crate::matrix::matrix::Matrix;
use crate::matrix::vector::Vector;
use crate::transform;

pub struct Camera {
	pub view: Matrix<f32, 4, 4>,
	pub position: Vector<f32, 3>,
}

impl Camera {
	pub fn new() -> Self {
		let up_vector = Vector::new([[0.0, 1.0, 0.0]]);
		let position = Vector::new([[0.0, 0.0, -3.5]]);
		let target = Vector::new([[0.0, 0.0, 0.0]]);
		let world = transform::look(position, target, up_vector);
		let view = world.inverse().unwrap();
		Camera { view, position }
	}
}
