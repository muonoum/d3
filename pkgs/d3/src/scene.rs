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
	pub objects: Vec<Object>,
	pub lights: Vec<Light>,
	pub ambient_color: Array<f32, 3>,
	pub camera: Camera,
}

impl Scene {
	pub fn load(path: &str) -> Scene {
		let scene_data = std::fs::read_to_string(path).unwrap();
		let scene = scene_data.parse::<toml::Table>().unwrap();

		let ambient_color = scene["ambient_color"].as_array().unwrap();
		let ar = ambient_color[0].as_float().unwrap() as f32;
		let ag = ambient_color[1].as_float().unwrap() as f32;
		let ab = ambient_color[2].as_float().unwrap() as f32;
		let ambient_color = array![ar, ag, ab];

		let camera = scene["camera"].as_table().unwrap();
		let camera_position = camera["position"].as_array().unwrap();
		let px = camera_position[0].as_float().unwrap() as f32;
		let py = camera_position[1].as_float().unwrap() as f32;
		let pz = camera_position[2].as_float().unwrap() as f32;
		let camera_target = camera["target"].as_array().unwrap();
		let tx = camera_target[0].as_float().unwrap() as f32;
		let ty = camera_target[1].as_float().unwrap() as f32;
		let tz = camera_target[2].as_float().unwrap() as f32;
		let camera = Camera::new(vector![px, py, pz], vector![tx, ty, tz]);

		let mut lights = vec![];

		for light in scene["lights"].as_array().unwrap() {
			let position = light["position"].as_array().unwrap();
			let x = position[0].as_float().unwrap() as f32;
			let y = position[1].as_float().unwrap() as f32;
			let z = position[2].as_float().unwrap() as f32;

			let diffuse = light["diffuse"].as_array().unwrap();
			let dr = diffuse[0].as_float().unwrap() as f32;
			let dg = diffuse[1].as_float().unwrap() as f32;
			let db = diffuse[2].as_float().unwrap() as f32;

			let specular = light["specular"].as_array().unwrap();
			let sr = specular[0].as_float().unwrap() as f32;
			let sg = specular[1].as_float().unwrap() as f32;
			let sb = specular[1].as_float().unwrap() as f32;

			lights.push(Light {
				position: vector![x, y, z],
				diffuse: array![dr, dg, db],
				specular: array![sr, sg, sb],
			});
		}

		let mut objects = vec![];

		for object in scene["objects"].as_array().unwrap() {
			let mesh = mesh::load(object["mesh"].as_str().unwrap()).unwrap();

			let position = object["position"].as_array().unwrap();
			let px = position[0].as_float().unwrap() as f32;
			let py = position[1].as_float().unwrap() as f32;
			let pz = position[2].as_float().unwrap() as f32;
			let position = vector![px, py, pz];

			let scale = object["scale"].as_array().unwrap();
			let sx = scale[0].as_float().unwrap() as f32;
			let sy = scale[1].as_float().unwrap() as f32;
			let sz = scale[2].as_float().unwrap() as f32;
			let scale = vector![sx, sy, sz];

			let orientation = object["orientation"].as_array().unwrap();
			let ox = orientation[0].as_float().unwrap() as f32;
			let oy = orientation[1].as_float().unwrap() as f32;
			let oz = orientation[2].as_float().unwrap() as f32;
			let orientation = vector![ox, oy, oz];

			let material = object["material"].as_table().unwrap();

			let emissive = material
				.get("emissive")
				.and_then(|v| v.as_array())
				.map(|emissive| {
					let r = emissive[0].as_float().unwrap() as f32;
					let g = emissive[1].as_float().unwrap() as f32;
					let b = emissive[2].as_float().unwrap() as f32;
					array![r, g, b]
				});

			let ambient = material
				.get("ambient")
				.and_then(|v| v.as_array())
				.map(|ambient| {
					let r = ambient[0].as_float().unwrap() as f32;
					let g = ambient[1].as_float().unwrap() as f32;
					let b = ambient[2].as_float().unwrap() as f32;
					array![r, g, b]
				});

			let diffuse = material
				.get("diffuse")
				.and_then(|v| v.as_array())
				.map(|diffuse| {
					let r = diffuse[0].as_float().unwrap() as f32;
					let g = diffuse[1].as_float().unwrap() as f32;
					let b = diffuse[2].as_float().unwrap() as f32;
					array![r, g, b]
				});

			let specular = material
				.get("specular")
				.and_then(|v| v.as_array())
				.map(|specular| {
					let r = specular[0].as_float().unwrap() as f32;
					let g = specular[1].as_float().unwrap() as f32;
					let b = specular[2].as_float().unwrap() as f32;
					array![r, g, b]
				});

			let shininess = material["shininess"].as_float().unwrap() as f32;

			let material = Material {
				emissive: emissive.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
				ambient: ambient.unwrap_or_else(|| array![0.1, 0.1, 0.1]),
				diffuse: diffuse.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
				specular: specular.unwrap_or_else(|| array![0.0, 0.0, 0.0]),
				shininess,
			};

			let update = object["update"].as_table().unwrap();
			let update_orientation = update["orientation"].as_array().unwrap();
			let uox = update_orientation[0].as_float().unwrap() as f32;
			let uoy = update_orientation[1].as_float().unwrap() as f32;
			let uoz = update_orientation[2].as_float().unwrap() as f32;

			let update = object::Update {
				orientation: vector![uox, uoy, uoz],
			};

			objects.push(Object {
				material,
				mesh,
				orientation,
				position,
				scale,
				update,
			});
		}

		Scene {
			ambient_color,
			camera,
			lights,
			objects,
		}
	}
}
