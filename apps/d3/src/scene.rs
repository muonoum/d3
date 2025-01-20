use std::time;

use array::{Array, array};
use matrix::{Vector, vector};
use render::light::Light;

use crate::camera::Camera;
use crate::object;
use crate::object::Object;

pub struct Scene {
	pub objects: Vec<Object>,
	pub lights: Vec<Light>,
	pub camera: Camera,
}

impl Scene {
	pub fn new(path: &str) -> Self {
		log::info!("Load {}", path);

		let table = std::fs::read_to_string(path)
			.unwrap()
			.parse::<toml::Table>()
			.unwrap();

		let camera = read_camera(table.get("camera").unwrap());
		let objects = read_objects(&table);
		let lights = read_lights(&table);

		Self {
			camera,
			objects,
			lights,
		}
	}

	pub fn update(
		&mut self,
		dt: time::Duration,
		movement: Vector<f32, 3>,
		orientation: Vector<f32, 2>,
	) {
		if movement != vector![0.0; 3] || orientation != vector![0.0; 2] {
			self.camera.update(dt, movement, orientation);
		}

		for object in self.objects.iter_mut() {
			object.update(dt);
		}
	}
}

fn read_lights(table: &toml::Table) -> Vec<Light> {
	table
		.get("lights")
		.and_then(|v| v.as_array())
		.map(|vs| vs.iter().map(read_light).collect())
		.unwrap_or_default()
}

fn read_objects(table: &toml::Table) -> Vec<Object> {
	table
		.get("objects")
		.and_then(|v| v.as_array())
		.unwrap()
		.iter()
		.map(read_object)
		.collect()
}

fn read_object(table: &toml::Value) -> Object {
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
		let orientation = if let Some(v) = table.get("orientation") {
			read_vector(v).unwrap()
		} else {
			vector![0.0; 3]
		};

		object::Update { orientation }
	});

	Object::new(path, scale, orientation, position, update)
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
	let pitch = table
		.get("pitch")
		.map(|v| v.as_float().unwrap() as f32)
		.unwrap_or(0.0);
	let yaw = table
		.get("yaw")
		.map(|v| v.as_float().unwrap() as f32)
		.unwrap_or(-90.0);
	Camera::new(position, pitch, yaw)
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

	// let object = table
	// 	.get("mesh")
	// 	.and_then(|v| v.as_str())
	// 	.map(|path| Object::new(path, vector![1.0; 3], vector![0.0; 3], position, None));

	Light {
		diffuse_color,
		specular_color,
		position,
		// object,
	}
}
