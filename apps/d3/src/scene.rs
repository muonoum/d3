use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::object;
use crate::object::Object;
use array::array;
use array::Array;
use matrix::vector;
use matrix::vector::Vector;

#[derive(Debug)]
pub struct Scene {
	pub ambient_color: Array<f32, 3>,
	pub lights: Vec<Light>,
	pub camera: Camera,
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn load(path: &str) -> Scene {
		let scene_data = std::fs::read_to_string(path).unwrap();
		let scene = scene_data.parse::<toml::Table>().unwrap();

		let ambient_color = {
			scene
				.get("ambient_color")
				.and_then(|v| get_triplet2(v, |r, g, b| array![r, g, b]))
				.unwrap_or_else(|| array![0.0; 3])
		};

		let camera = {
			let table = scene.get("camera").and_then(|v| v.as_table()).unwrap();

			let position = table
				.get("position")
				.and_then(|v| get_triplet2(v, |x, y, z| vector![x, y, z]))
				.unwrap();

			let target = table
				.get("target")
				.and_then(|v| get_triplet2(v, |x, y, z| vector![x, y, z]))
				.unwrap();

			Camera::new(position, target)
		};

		let lights = scene.get("lights").map(|lights| {
			lights
				.as_array()
				.unwrap()
				.iter()
				.map(|light| {
					let table = light.as_table().unwrap();

					let position =
						get_triplet(table, "position", |x, y, z| vector![x, y, z]).unwrap();

					let ambient_color =
						get_triplet(table, "ambient_color", |r, g, b| array![r, g, b]);
					let diffuse_color =
						get_triplet(table, "diffuse_color", |r, g, b| array![r, g, b]);
					let specular_color =
						get_triplet(table, "specular_color", |r, g, b| array![r, g, b]);

					Light {
						ambient_color: ambient_color.unwrap_or_else(|| array![0.0; 3]),
						position,
						diffuse_color: diffuse_color.unwrap_or_else(|| array![0.0; 3]),
						specular_color: specular_color.unwrap_or_else(|| array![0.0; 3]),
					}
				})
				.collect()
		});

		let objects = scene["objects"]
			.as_array()
			.unwrap()
			.iter()
			.map(|object| {
				let table = object.as_table().unwrap();
				let mesh = obj::Mesh::new(table["mesh"].as_str().unwrap()).unwrap();

				let position = get_triplet(table, "position", |x, y, z| vector![x, y, z]);
				let scale = get_triplet(table, "scale", |x, y, z| vector![x, y, z]);
				let orientation = get_triplet(table, "orientation", |x, y, z| vector![x, y, z]);

				let material = object
					.get("material")
					.and_then(|value| value.as_table().map(get_material));

				let update = object.get("update").and_then(|value| {
					value.as_table().map(|table| {
						let orientation =
							get_triplet(table, "orientation", |x, y, z| vector![x, y, z])
								.unwrap();
						object::Update { orientation }
					})
				});

				Object {
					material: material.unwrap_or_default(),
					mesh,
					orientation: orientation.unwrap_or_else(Vector::zero),
					position: position.unwrap_or_else(Vector::zero),
					scale: scale.unwrap_or_else(|| vector![1.0; 3]),
					update,
				}
			})
			.collect();

		Scene {
			ambient_color,
			camera,
			lights: lights.unwrap_or_else(Vec::new),
			objects,
		}
	}
}

fn get_triplet2<T>(value: &toml::Value, f: impl Fn(f32, f32, f32) -> T) -> Option<T> {
	value.as_array().map(|vs| {
		let a = vs[0].as_float().unwrap() as f32;
		let b = vs[1].as_float().unwrap() as f32;
		let c = vs[2].as_float().unwrap() as f32;
		f(a, b, c)
	})
}

fn get_triplet<T>(table: &toml::Table, key: &str, f: impl Fn(f32, f32, f32) -> T) -> Option<T> {
	table.get(key).and_then(|v| v.as_array()).map(|vs| {
		let a = vs[0].as_float().unwrap() as f32;
		let b = vs[1].as_float().unwrap() as f32;
		let c = vs[2].as_float().unwrap() as f32;
		f(a, b, c)
	})
}

fn get_material(table: &toml::Table) -> Material {
	let emissive = get_triplet(table, "emissive_color", |r, g, b| array![r, g, b]);
	let ambient = get_triplet(table, "ambient_reflection", |r, g, b| array![r, g, b]);
	let diffuse = get_triplet(table, "diffuse_reflection", |r, g, b| array![r, g, b]);
	let specular = get_triplet(table, "specular_reflection", |r, g, b| array![r, g, b]);
	let exponent = table
		.get("specular_exponent")
		.and_then(|value| value.as_integer().map(|n| n as i32));

	Material {
		emissive_color: emissive.unwrap_or_else(|| array![0.0; 3]),
		ambient_reflection: ambient.unwrap_or_else(|| array![0.0; 3]),
		diffuse_reflection: diffuse.unwrap_or_else(|| array![0.0; 3]),
		specular_reflection: specular.unwrap_or_else(|| array![0.0; 3]),
		specular_exponent: exponent.unwrap_or(1),
	}
}
