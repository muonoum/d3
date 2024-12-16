use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::mesh;
use crate::object;
use crate::object::Object;
use array::array;
use array::Array;
use matrix::vector;

#[derive(Debug)]
pub struct Scene {
	pub ambience: Array<f32, 3>,
	pub lights: Vec<Light>,
	pub camera: Camera,
	pub objects: Vec<Object>,
}

impl Scene {
	pub fn load(path: &str) -> Scene {
		let scene_data = std::fs::read_to_string(path).unwrap();
		let scene = scene_data.parse::<toml::Table>().unwrap();

		let ambience = {
			scene
				.get("ambience")
				.and_then(|v| v.as_array())
				.map(|vs| {
					let a = vs[0].as_float().unwrap() as f32;
					let b = vs[1].as_float().unwrap() as f32;
					let c = vs[2].as_float().unwrap() as f32;
					array![a, b, c]
				})
				.unwrap()
		};

		let camera = {
			let camera = scene["camera"].as_table().unwrap();
			let position = get_triplet(camera, "position", |x, y, z| vector![x, y, z]).unwrap();
			let target = get_triplet(camera, "target", |x, y, z| vector![x, y, z]).unwrap();
			Camera::new(position, target)
		};

		let lights = scene["lights"]
			.as_array()
			.unwrap()
			.iter()
			.map(|light| {
				let table = light.as_table().unwrap();

				let position = get_triplet(table, "position", |x, y, z| vector![x, y, z]).unwrap();
				let ambient_color = get_triplet(table, "ambient_color", |r, g, b| array![r, g, b]);
				let diffuse_color =
					get_triplet(table, "diffuse_color", |r, g, b| array![r, g, b]).unwrap();
				let specular_color =
					get_triplet(table, "specular_color", |r, g, b| array![r, g, b]).unwrap();

				Light {
					ambient_color: ambient_color.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
					position,
					diffuse_color,
					specular_color,
				}
			})
			.collect();

		let objects = scene["objects"]
			.as_array()
			.unwrap()
			.iter()
			.map(|object| {
				let table = object.as_table().unwrap();
				let mesh = mesh::load(table["mesh"].as_str().unwrap()).unwrap();

				let position = get_triplet(table, "position", |x, y, z| vector![x, y, z]).unwrap();
				let scale = get_triplet(table, "scale", |x, y, z| vector![x, y, z]).unwrap();
				let orientation =
					get_triplet(table, "orientation", |x, y, z| vector![x, y, z]).unwrap();

				let material = object["material"].as_table().map(get_material).unwrap();

				let update = {
					let table = object["update"].as_table().unwrap();
					let orientation =
						get_triplet(table, "orientation", |x, y, z| vector![x, y, z]).unwrap();
					object::Update { orientation }
				};

				Object {
					material,
					mesh,
					orientation,
					position,
					scale,
					update,
				}
			})
			.collect();

		Scene {
			ambience,
			camera,
			lights,
			objects,
		}
	}
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
	let exponent = table["specular_exponent"].as_float().unwrap() as f32;

	Material {
		emissive_color: emissive.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
		ambient_reflection: ambient.unwrap_or_else(|| array![0.1, 0.1, 0.1]),
		diffuse_reflection: diffuse.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
		specular_reflection: specular.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
		specular_exponent: exponent,
	}
}
