use crate::light::Light;
use crate::material::Material;
use array::Array;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Phong,
	BlinnPhong,
}

pub trait Reflect {
	fn reflect(
		&self,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3>;
}

impl Reflect for Model {
	fn reflect(
		&self,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		match self {
			Model::Phong => Phong::reflect(position, normal, lights, material, camera),
			Model::BlinnPhong => BlinnPhong::reflect(position, normal, lights, material, camera),
		}
	}
}

pub struct Phong {}

impl Phong {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		lights.iter().fold(material.emissive, |sum, light| {
			sum + Self::light(light, position, normal, material, camera)
		}) * 255.0
	}

	fn light(
		light: &Light,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let light_dir = (light.position - position).normalize();
		let camera_dir = (camera - position).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);

		let specular = if diffuse == 0.0 {
			0.0
		} else {
			// ((normal * light_dir.dot(normal) * 2.0) - light_dir)
			light_dir
				.reflect(normal)
				.normalize()
				.dot(camera_dir)
				.clamp(0.0, 1.0)
				.powf(material.shininess)
		};

		material.ambient * light.ambient
			+ material.diffuse * diffuse * light.diffuse
			+ material.specular * specular * light.specular
	}
}

pub struct BlinnPhong {}

impl BlinnPhong {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		lights.iter().fold(material.emissive, |sum, light| {
			sum + Self::light(light, position, normal, material, camera)
		}) * 255.0
	}

	fn light(
		light: &Light,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let light_dir = (light.position - position).normalize();
		let camera_dir = (camera - position).normalize();
		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);

		let specular = if diffuse == 0.0 {
			0.0
		} else {
			normal
				.dot((light_dir + camera_dir).normalize())
				.clamp(0.0, 1.0)
				.powf(material.shininess)
		};

		material.ambient * light.ambient
			+ material.diffuse * diffuse * light.diffuse
			+ material.specular * specular * light.specular
	}
}
