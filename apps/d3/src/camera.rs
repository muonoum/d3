use matrix::{Matrix, Vector, transform, vector};

pub struct Camera {
	pub position: Vector<f32, 3>,
	pub target: Vector<f32, 3>,
	pub view: Matrix<f32, 4, 4>,
}

impl Camera {
	pub fn new(position: Vector<f32, 3>, target: Vector<f32, 3>) -> Self {
		let world = transform::look_at(position, target, vector![0.0, 1.0, 0.0]);
		let view = world.inverse().unwrap();

		Camera {
			position,
			target,
			view,
		}
	}

	pub fn update(&mut self, movement: Vector<f32, 3>) {
		self.position += movement;
		self.target += movement;
		let up_vector = vector![0.0, 1.0, 0.0];
		let world = transform::look_at(self.position, self.target, up_vector);
		self.view = world.inverse().unwrap();
	}
}
