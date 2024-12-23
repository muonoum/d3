use crate::camera::Camera;
use array::{array, Array};
use matrix::{vector, Matrix, Vector};
use render::Pipeline;

pub type Vertex = (Vector<f32, 3>, Array<f32, 3>);

pub struct Scene {
	pub object: Vec<[Vertex; 3]>,
	pub camera: Camera,
	pub projection: Matrix<f32, 4, 4>,
}

impl Scene {
	pub fn new(width: usize, height: usize) -> Self {
		let object = vec![[
			(vector![-1.0, 2.0, 0.0], array![1.0, 0.0, 0.0]),
			(vector![-2.0, -1.0, 0.0], array![0.0, 0.0, 1.0]),
			(vector![1.0, -1.0, 2.0], array![0.0, 1.0, 0.0]),
		]];

		let projection = transform::perspective_near(width as f32 / height as f32, 2.0, 0.1);
		// transform::perspective_near_far(width as f32 / height as f32, 50.0, 0.1, 100.0);
		let camera = Camera::new(vector![0.0, 0.0, 4.5], vector![0.0, 0.0, 0.0]);

		Self {
			object,
			camera,
			projection,
		}
	}
}

impl Pipeline for &Scene {
	type Setup = ();
	type Vertex = Vertex;
	type Fragment = [u8; 4];
	type Varying = Array<f32, 3>;
	type Face = [Self::Vertex; 3];

	fn setup(&self) -> Self::Setup {}

	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3] {
		*face
	}

	fn vertex(
		&self,
		vertex: &Self::Vertex,
		_setup: &Self::Setup,
	) -> (Vector<f32, 4>, Self::Varying) {
		let (position, color) = vertex;
		let world = position.v4();
		let clip = world * self.camera.view * self.projection;
		(clip, *color)
	}

	fn fragment(&self, _face: &Self::Face, data: &Self::Varying) -> Self::Fragment {
		[
			(data[0] * 255.0) as u8,
			(data[1] * 255.0) as u8,
			(data[2] * 255.0) as u8,
			255,
		]
	}
}
