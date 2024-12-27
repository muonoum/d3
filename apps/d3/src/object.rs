use matrix::{transform, Matrix, Vector};

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

		log::info!(
			"Load {}: f={}; v={}; vn={}",
			path,
			mesh.groups.iter().map(|g| g.faces.len()).sum::<usize>(),
			mesh.positions.len(),
			mesh.normals.len(),
		);

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

// pub struct Render<'a> {
// 	pub object: &'a Object,
// 	pub material: &'a Option<Arc<obj::Material>>,
// 	pub camera: &'a Camera,
// 	pub lights: &'a Vec<Light>,
// 	pub projection: Matrix<f32, 4, 4>,
// }

// impl render::Pipeline for Render<'_> {
// 	type Setup = (Vec<(Vector<f32, 3>, Vector<f32, 4>)>, Vec<Vector<f32, 3>>);
// 	type Face = obj::Face;
// 	type Fragment = [u8; 4];
// 	type Vertex = obj::Vertex;
// 	type Attributes = (
// 		Vector<f32, 3>,
// 		Option<Vector<f32, 3>>,
// 		Option<Vector<f32, 2>>,
// 	);

// 	fn setup(&self) -> Self::Setup {
// 		let clip_space = self.camera.view * self.projection;

// 		let positions = self.object.mesh.positions.iter().map(|v| {
// 			let world = v.v4() * self.object.world_space;
// 			(world.v3(), world * clip_space)
// 		});

// 		let normals = self
// 			.object
// 			.mesh
// 			.normals
// 			.iter()
// 			.map(|v| *v * self.object.normal_space);

// 		(positions.collect(), normals.collect())
// 	}

// 	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3] {
// 		*face
// 	}

// 	fn vertex(
// 		&self,
// 		vertex: &Self::Vertex,
// 		setup: &Self::Setup,
// 	) -> (Vector<f32, 4>, Self::Attributes) {
// 		let (positions, normals) = setup;
// 		let (world, clip) = positions[vertex.position];
// 		let uv = vertex.texture.map(|i| self.object.mesh.textures[i]);
// 		let normal = vertex.normal.map(|i| normals[i]);
// 		(clip, (world, normal, uv))
// 	}

// 	fn fragment(&self, _face: &Self::Face, attrs: &Self::Attributes) -> Self::Fragment {
// 		let (position, normal, uv) = attrs;

// 		match (self.material.clone(), uv, normal) {
// 			(Some(material), Some(uv), Some(normal)) => {
// 				let diffuse_map = if let Some(ref map) = material.diffuse_map {
// 					map_texture(map, uv)
// 				} else {
// 					array![1.0; 3]
// 				};

// 				let color = blinn_phong(
// 					material,
// 					diffuse_map,
// 					*position,
// 					normal.normalize(),
// 					self.camera.position,
// 					self.lights,
// 				);

// 				[color[0] as u8, color[1] as u8, color[2] as u8, 255]
// 			}

// 			(Some(material), None, Some(normal)) => {
// 				let color = blinn_phong(
// 					material,
// 					array![1.0; 3],
// 					*position,
// 					normal.normalize(),
// 					self.camera.position,
// 					self.lights,
// 				);

// 				[color[0] as u8, color[1] as u8, color[2] as u8, 255]
// 			}

// 			_else => [255, 0, 255, 255],
// 		}
// 	}
// }

// pub fn map_texture(texture: &image::RgbImage, uv: &Vector<f32, 2>) -> Array<f32, 3> {
// 	let (width, height) = (texture.width() as f32, texture.height() as f32);
// 	let x = (0.0f32).max(uv[0] * width).min(width - 1.0);
// 	let y = (0.0f32).max(uv[1] * height).min(height - 1.0);
// 	let rgb = texture.get_pixel(x as u32, y as u32);

// 	array![
// 		rgb[0] as f32 / 255.0,
// 		rgb[1] as f32 / 255.0,
// 		rgb[2] as f32 / 255.0
// 	]
// }

// pub fn blinn_phong(
// 	material: Arc<obj::Material>,
// 	diffuse_map: Array<f32, 3>,
// 	position: Vector<f32, 3>,
// 	normal: Vector<f32, 3>,
// 	camera_position: Vector<f32, 3>,
// 	lights: &[Light],
// ) -> Array<f32, 3> {
// 	let camera_dir = (camera_position - position).normalize();

// 	lights.iter().fold(array![0.0; 3], |sum, light| {
// 		let light_dir = (light.position - position).normalize();
// 		let halfway_vector = (light_dir + camera_dir).normalize();
// 		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
// 		let specular = normal
// 			.dot(halfway_vector)
// 			.powi(material.specular_exponent as i32);
// 		sum + material.diffuse * diffuse_map * diffuse * light.diffuse_color
// 			+ material.specular * specular * light.specular_color
// 	}) * 255.0
// }
