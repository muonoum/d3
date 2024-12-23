use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use array::{array, Array};
use matrix::{Matrix, Vector};

pub struct Object {
	pub mesh: obj::Mesh,
	pub texture: Option<image::RgbImage>,
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
		texture_path: Option<&str>,
		scale: Vector<f32, 3>,
		orientation: Vector<f32, 3>,
		position: Vector<f32, 3>,
		material: Material,
		update: Option<Update>,
	) -> Self {
		let mesh = obj::Mesh::new(path).unwrap();
		let texture = texture_path.map(|path| image::open(path).unwrap().to_rgb8());
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
			texture,
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
	type Setup = (Vec<(Vector<f32, 3>, Vector<f32, 4>)>, Vec<Vector<f32, 3>>);

	type Face = obj::Face;
	type Fragment = [u8; 4];
	type Vertex = obj::Vertex;
	type Attributes = (Vector<f32, 3>, Vector<f32, 3>, Option<Vector<f32, 2>>);

	fn setup(&self) -> Self::Setup {
		let clip_space = self.camera.view * self.projection;

		let positions = self.object.mesh.positions.iter().map(|v| {
			let world = v.v4() * self.object.world_space;
			(world.v3(), world * clip_space)
		});

		let normals = self
			.object
			.mesh
			.normals
			.iter()
			.map(|v| *v * self.object.normal_space);

		(positions.collect(), normals.collect())
	}

	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3] {
		face.vertices
	}

	fn vertex(
		&self,
		vertex: &Self::Vertex,
		setup: &Self::Setup,
	) -> (Vector<f32, 4>, Self::Attributes) {
		let uv = vertex.texture.map(|i| self.object.mesh.texture[i]);
		let (positions, normals) = setup;
		let (world, clip) = positions[vertex.position];
		(clip, (world, normals[vertex.normal], uv))
	}

	fn fragment(&self, face: &Self::Face, attrs: &Self::Attributes) -> Self::Fragment {
		let (position, normal, uv) = attrs;

		// TODO
		match (&self.object.texture, uv) {
			(Some(texture), Some(uv)) => {
				let (width, height) = (texture.width() as f32, texture.height() as f32);
				let x = f32::min(f32::max(0.0, uv[0] * width), width - 1.0);
				let y = f32::min(f32::max(0.0, uv[1] * height), height - 1.0);
				let rgb = texture.get_pixel(x as u32, y as u32);
				let diffuse = array![
					rgb[0] as f32 / 255.0,
					rgb[1] as f32 / 255.0,
					rgb[2] as f32 / 255.0
				];
				let color = blinn_phong2(
					diffuse,
					match &face.material {
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

			_else => {
				let color = blinn_phong(
					match &face.material {
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

// TODO
fn blinn_phong2(
	diffuse_component: Array<f32, 3>,
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
		sum + diffuse_component * diffuse * light.diffuse_color
			+ material.specular_component * specular * light.specular_color
	}) * 255.0
}
