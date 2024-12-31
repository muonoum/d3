use crate::camera::Camera;
use crate::light::Light;
use crate::object;
use crate::object::Object;
use array::{Array, array};
use matrix::{Matrix, Vector, vector};

pub struct Scene {
	pub objects: Vec<Object>,
	pub lights: Vec<Light>,
	pub camera: Camera,
	pub projection: Matrix<f32, 4, 4>,
}

impl Scene {
	pub fn new(path: &str, projection: Matrix<f32, 4, 4>) -> Self {
		log::info!("Load {}", path);

		let table = std::fs::read_to_string(path)
			.unwrap()
			.parse::<toml::Table>()
			.unwrap();

		let camera = read_camera(table.get("camera").unwrap());

		let objects = table
			.get("objects")
			.and_then(|v| v.as_array())
			.unwrap()
			.iter()
			.map(|table| {
				let path = table.get("mesh").unwrap().as_str().unwrap();

				let scale = if let Some(v) = table.get("scale") {
					read_vector(v).unwrap()
				} else {
					vector![1.0; 3]
				};

				let orientation = if let Some(v) = table.get("orientation") {
					read_vector(v).unwrap()
				} else {
					vector![0.0; 3]
				};

				let position = if let Some(v) = table.get("position") {
					read_vector(v).unwrap()
				} else {
					vector![0.0; 3]
				};

				let update = table.get("update").map(|table| {
					let orientation = table.get("orientation").and_then(read_vector).unwrap();
					object::Update { orientation }
				});

				Object::new(path, scale, orientation, position, update)
			});

		let lights = table
			.get("lights")
			.and_then(|v| v.as_array())
			.unwrap()
			.iter()
			.map(read_light);

		Self {
			camera,
			projection,
			objects: objects.collect(),
			lights: lights.collect(),
		}
	}

	pub fn update(&mut self, movement: Vector<f32, 3>) {
		if movement != vector![0.0; 3] {
			self.camera.update(movement);
		}

		for object in self.objects.iter_mut() {
			object.update();
		}
	}
}

fn read_vector(value: &toml::Value) -> Option<Vector<f32, 3>> {
	read_triplet(value, |x, y, z| vector![x, y, z])
}

fn read_array(value: &toml::Value) -> Option<Array<f32, 3>> {
	read_triplet(value, |a, b, c| array![a, b, c])
}

fn read_triplet<T>(value: &toml::Value, f: impl Fn(f32, f32, f32) -> T) -> Option<T> {
	value.as_array().map(|vs| {
		let a = vs[0].as_float().unwrap() as f32;
		let b = vs[1].as_float().unwrap() as f32;
		let c = vs[2].as_float().unwrap() as f32;
		f(a, b, c)
	})
}

pub fn read_camera(table: &toml::Value) -> Camera {
	let position = table.get("position").and_then(read_vector).unwrap();
	let target = table.get("target").and_then(read_vector).unwrap();
	Camera::new(position, target)
}

pub fn read_light(table: &toml::Value) -> Light {
	let position = table.get("position").and_then(read_vector).unwrap();

	let diffuse_color = if let Some(v) = table.get("diffuse_color") {
		read_array(v).unwrap()
	} else {
		array![1.0; 3]
	};

	let specular_color = if let Some(v) = table.get("specular_color") {
		read_array(v).unwrap()
	} else {
		array![0.0; 3]
	};

	Light {
		position,
		diffuse_color,
		specular_color,
	}
}
