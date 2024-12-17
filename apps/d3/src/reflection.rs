use crate::light::Light;
use crate::material::Material;
use array::Array;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Phong,
	BlinnPhong,
}

impl Model {
	pub fn reflect(
		&self,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		ambient_color: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		match self {
			Model::Phong => {
				Phong::reflect(position, normal, ambient_color, lights, material, camera)
			}

			Model::BlinnPhong => {
				BlinnPhong::reflect(position, normal, ambient_color, lights, material, camera)
			}
		}
	}
}

#[allow(dead_code)]
fn phong_specular(
	light_dir: Vector<f32, 3>,
	camera_dir: Vector<f32, 3>,
	normal: Vector<f32, 3>,
) -> f32 {
	((normal * light_dir.dot(normal) * 2.0) - light_dir)
		.normalize()
		.dot(camera_dir)
		.clamp(0.0, 1.0)
}

#[allow(dead_code)]
fn blinn_specular(
	light_dir: Vector<f32, 3>,
	camera_dir: Vector<f32, 3>,
	normal: Vector<f32, 3>,
) -> f32 {
	normal.dot((light_dir + camera_dir).normalize())
}

pub struct Phong {}

impl Phong {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		ambient_color: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		lights.iter().fold(
			material.ambient_reflection * ambient_color + material.emissive_color,
			|sum, light| sum + Self::light(light, position, normal, material, camera_dir),
		) * 255.0
	}

	fn light(
		light: &Light,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		material: Material,
		camera_dir: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let light_dir = (light.position - position).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let specular = if diffuse > 0.0 {
			phong_specular(light_dir, camera_dir, normal).powi(material.specular_exponent)
		} else {
			0.0
		};

		material.ambient_reflection * light.ambient_color
			+ material.diffuse_reflection * diffuse * light.diffuse_color
			+ material.specular_reflection * specular * light.specular_color
	}
}

pub struct BlinnPhong {}

impl BlinnPhong {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		ambient_color: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		lights.iter().fold(
			material.ambient_reflection * ambient_color + material.emissive_color,
			|sum, light| sum + Self::light(light, position, normal, material, camera_dir),
		) * 255.0
	}

	fn light(
		light: &Light,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		material: Material,
		camera_dir: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let light_dir = (light.position - position).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let specular = if diffuse > 0.0 {
			blinn_specular(light_dir, camera_dir, normal).powi(material.specular_exponent * 4)
		} else {
			0.0
		};

		material.ambient_reflection * light.ambient_color
			+ material.diffuse_reflection * diffuse * light.diffuse_color
			+ material.specular_reflection * specular * light.specular_color
	}
}
