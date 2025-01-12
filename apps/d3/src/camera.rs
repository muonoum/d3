use std::time;

use matrix::{Matrix, Vector, vector};

pub struct Camera {
	pub speed: f32,
	pub sensitivity: f32,
	pub pitch: f32,
	pub yaw: f32,
	pub position: Vector<f32, 3>,
	pub target: Vector<f32, 3>,
	pub up: Vector<f32, 3>,
	pub view: Matrix<f32, 4, 4>,
}

impl Camera {
	pub fn new(position: Vector<f32, 3>, pitch: f32, yaw: f32) -> Self {
		let mut camera = Camera {
			speed: 3.5,
			sensitivity: 12.0,
			position,
			pitch,
			yaw,
			target: vector![0.0, 0.0, -1.0],
			up: vector![0.0, 1.0, 0.0],
			view: Matrix::identity(),
		};

		camera.update_matrix();
		camera
	}

	pub fn update(
		&mut self,
		dt: time::Duration,
		movement: Vector<f32, 3>,
		orientation: Vector<f32, 2>,
	) {
		let dt = dt.as_secs_f32();
		let right = self.target.cross(self.up).normalize();
		let forward = self.up.cross(right).normalize();

		self.position += right * movement[0] * self.speed * dt;
		self.position += self.up * movement[1] * self.speed * dt;
		self.position += forward * movement[2] * self.speed * dt;

		self.yaw += orientation[0] * self.sensitivity * dt;
		self.pitch -= orientation[1] * self.sensitivity * dt;
		self.pitch = self.pitch.clamp(-89.0, 89.0);

		self.update_matrix();
	}

	pub fn update_matrix(&mut self) {
		let (sin_yaw, cos_yaw) = self.yaw.to_radians().sin_cos();
		let (sin_pitch, cos_pitch) = self.pitch.to_radians().sin_cos();
		self.target = vector![cos_yaw * cos_pitch, sin_pitch, sin_yaw * cos_pitch].normalize();

		let world = transform::look_at(self.position, self.position + self.target, self.up);
		self.view = world.inverse().unwrap();
	}
}
