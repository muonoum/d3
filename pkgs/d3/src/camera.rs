use crate::transform;
use matrix::matrix::Matrix;
use matrix::vector::Vector;

#[derive(Debug)]
pub struct Camera {
	pub view: Matrix<f32, 4, 4>,
	pub position: Vector<f32, 3>,
	pub target: Vector<f32, 3>,
}

impl Camera {
	pub fn new(position: Vector<f32, 3>, target: Vector<f32, 3>) -> Self {
		let up_vector = Vector::new([[0.0, 1.0, 0.0]]);
		let world = transform::look(position, target, up_vector);
		let view = world.inverse().unwrap();

		Camera {
			view,
			position,
			target,
		}
	}

	pub fn update_camera(&mut self, movement: Vector<f32, 3>, target: Vector<f32, 3>) {
		self.position += movement;
		// TODO: Virker ikke
		self.target += target;
		let up_vector = Vector::new([[0.0, 1.0, 0.0]]);
		let world = transform::look(self.position, self.target, up_vector);
		self.view = world.inverse().unwrap();
	}
}
