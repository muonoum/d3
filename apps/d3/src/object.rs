use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use array::{array, Array};
use matrix::{Matrix, Vector};

pub struct Object {
	pub mesh: obj::Mesh,
	pub material: Material,
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
		material: Material,
		update: Option<Update>,
	) -> Self {
		let mesh = obj::Mesh::new(path).unwrap();
		let world_space = transform::scale_vector(scale)
			* transform::rotate_vector(orientation)
			* transform::translate_vector(position);
		let normal_space = world_space.sub_matrix(3, 3).unwrap();

		log::info!(
			"Load {}: f={}; v={}; vn={}",
			path,
			mesh.faces.len(),
			mesh.positions.len(),
			mesh.normals.len(),
		);

		Object {
			mesh,
			material,
			scale,
			orientation,
			position,
			normal_space,
			world_space,
			update,
		}
	}

	pub fn update(&mut self) {
		if let Some(update) = &self.update {
			self.orientation += update.orientation;
			self.world_space = transform::scale_vector(self.scale)
				* transform::rotate_vector(self.orientation)
				* transform::translate_vector(self.position);
			self.normal_space = self.world_space.sub_matrix(3, 3).unwrap();
		}
	}
}

pub struct Render<'a> {
	pub object: &'a Object,
	pub camera: &'a Camera,
	pub lights: &'a Vec<Light>,
	pub projection: Matrix<f32, 4, 4>,
}

impl render::Pipeline for Render<'_> {
	type Setup = (
		Vec<Vector<f32, 3>>,
		Vec<Vector<f32, 4>>,
		Vec<Vector<f32, 3>>,
	);

	type Face = obj::Face;
	type Fragment = [u8; 4];
	type Vertex = obj::Vertex;
	type Varying = (Vector<f32, 3>, Vector<f32, 3>);

	fn prepare(&self) -> Self::Setup {
		let clip_space = self.camera.view * self.projection;

		let mut world_positions = vec![];
		let mut clip_positions = vec![];
		let mut normals = vec![];

		for v in self.object.mesh.positions.iter() {
			let world = v.v4() * self.object.world_space;
			world_positions.push(world.v3());
			clip_positions.push(world * clip_space);
		}

		for v in self.object.mesh.normals.iter() {
			normals.push(*v * self.object.normal_space);
		}

		(world_positions, clip_positions, normals)
	}

	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3] {
		face.vertices
	}

	fn vertex(
		&self,
		vertex: &Self::Vertex,
		setup: &Self::Setup,
	) -> (Vector<f32, 4>, Self::Varying) {
		let (world, clip, normals) = setup;

		(
			clip[vertex.position],
			(world[vertex.position], normals[vertex.normal]),
		)
	}

	fn fragment(&self, face: &Self::Face, (position, normal): &Self::Varying) -> Self::Fragment {
		let color = blinn_phong(
			match face.material {
				Some(material) => material.into(),
				None => self.object.material,
			},
			*position,
			normal.normalize(),
			self.camera.position,
			self.lights,
		);

		[color[0] as u8, color[1] as u8, color[2] as u8, 255]
	}
}

fn blinn_phong(
	material: Material,
	position: Vector<f32, 3>,
	normal: Vector<f32, 3>,
	camera_position: Vector<f32, 3>,
	lights: &[Light],
) -> Array<f32, 3> {
	let camera_dir = (camera_position - position).normalize();

	lights.iter().fold(array![0.0; 3], |sum, light| {
		let light_dir = (light.position - position).normalize();
		let halfway_vector = (light_dir + camera_dir).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let specular = normal.dot(halfway_vector).powi(material.specular_exponent);
		sum + material.diffuse_component * diffuse * light.diffuse_color
			+ material.specular_component * specular * light.specular_color
	}) * 255.0
}
