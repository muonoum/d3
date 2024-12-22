use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::object;
use crate::object::Object;
use array::{array, Array};
use matrix::{vector, Vector};

pub struct Scene {
	pub objects: Vec<Object>,
	pub lights: Vec<Light>,
	pub camera: Camera,
}

impl Scene {
	pub fn new(path: &str, width: usize, height: usize) -> Self {
		let scene_data = std::fs::read_to_string(path).unwrap();
		let table = scene_data.parse::<toml::Table>().unwrap();

		let camera = {
			let table = table.get("camera").unwrap();
			let position = table.get("position").and_then(read_vector).unwrap();
			let target = table.get("target").and_then(read_vector).unwrap();
			let projection = transform::perspective_near(width as f32 / height as f32, 2.0, 0.1);
			Camera::new(position, target, projection)
		};

		let objects = table
			.get("objects")
			.and_then(|v| v.as_array())
			.unwrap()
			.iter()
			.map(|value| {
				let table = value.as_table().unwrap();
				let path = table.get("mesh").and_then(|v| v.as_str()).unwrap();
				let scale = table.get("scale").and_then(read_vector).unwrap();
				let orientation = table.get("orientation").and_then(read_vector).unwrap();
				let position = table.get("position").and_then(read_vector).unwrap();

				let material = table.get("material").map(|table| {
					let diffuse_component = table
						.get("diffuse_component")
						.and_then(read_array)
						.unwrap_or_else(|| array![1.0; 3]);

					let specular_component = table
						.get("specular_component")
						.and_then(read_array)
						.unwrap_or_else(|| array![0.0; 3]);

					let specular_exponent = table
						.get("specular_exponent")
						.and_then(|value| value.as_integer())
						.unwrap_or(0);

					Material {
						diffuse_component,
						specular_component,
						specular_exponent: specular_exponent as i32,
					}
				});

				let update = table.get("update").and_then(|value| {
					value.as_table().map(|table| {
						let orientation = table.get("orientation").and_then(read_vector).unwrap();
						object::Update { orientation }
					})
				});

				Object::new(
					path,
					scale,
					orientation,
					position,
					material.unwrap_or_default(),
					update,
				)
			});

		let lights = table
			.get("lights")
			.and_then(|v| v.as_array())
			.unwrap()
			.iter()
			.map(|value| {
				let table = value.as_table().unwrap();
				let position = table.get("position").and_then(read_vector).unwrap();

				let diffuse_color = table
					.get("diffuse_color")
					.and_then(read_array)
					.unwrap_or_else(|| array![1.0; 3]);

				let specular_color = table
					.get("specular_color")
					.and_then(read_array)
					.unwrap_or_else(|| array![0.0; 3]);

				Light {
					position,
					diffuse_color,
					specular_color,
				}
			});

		Self {
			camera,
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
