use std::time;

use matrix::{Matrix, Vector};

pub struct Object {
	pub mesh: obj::Mesh,
	pub scale: Vector<f32, 3>,
	pub orientation: Vector<f32, 3>,
	pub position: Vector<f32, 3>,
	pub world_space: Matrix<f32, 4, 4>,
	pub normal_space: Matrix<f32, 3, 3>,
	pub update: Option<Update>,
}

pub struct Update {
	pub orientation: Vector<f32, 3>,
}

impl Object {
	pub fn new(
		path: &str,
		scale: Vector<f32, 3>,
		orientation: Vector<f32, 3>,
		position: Vector<f32, 3>,
		update: Option<Update>,
	) -> Self {
		let mesh = obj::Mesh::new(path).unwrap();
		let world_space = transform::scale_vector(scale)
			* transform::rotate_vector(orientation)
			* transform::translate_vector(position);
		let normal_space = world_space.sub_matrix(3, 3).unwrap();

		Object {
			mesh,
			scale,
			orientation,
			position,
			normal_space,
			world_space,
			update,
		}
	}

	pub fn update(&mut self, dt: time::Duration) {
		if let Some(update) = &self.update {
			let dt = dt.as_secs_f32();

			self.orientation += update.orientation * dt;
			self.world_space = transform::scale_vector(self.scale)
				* transform::rotate_vector(self.orientation)
				* transform::translate_vector(self.position);
			self.normal_space = self.world_space.sub_matrix(3, 3).unwrap();
		}
	}
}
