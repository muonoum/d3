use crate::light::Light;
use crate::material::Material;
use array::array;
use array::Array;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Phong,
	BlinnPhong,
	Test,
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
			Model::Test => Test::reflect(position, normal, lights, material, camera),
		}
	}
}

#[allow(dead_code)]
fn phong(light_dir: Vector<f32, 3>, camera_dir: Vector<f32, 3>, normal: Vector<f32, 3>) -> f32 {
	((normal * light_dir.dot(normal) * 2.0) - light_dir)
		.normalize()
		.dot(camera_dir)
		.clamp(0.0, 1.0)
}

#[allow(dead_code)]
fn blinn_phong(
	light_dir: Vector<f32, 3>,
	camera_dir: Vector<f32, 3>,
	normal: Vector<f32, 3>,
) -> f32 {
	normal.dot((light_dir + camera_dir).normalize())
}

#[allow(dead_code)]
fn schlick(
	material_specular: Array<f32, 3>,
	light_dir: Vector<f32, 3>,
	normal: Vector<f32, 3>,
) -> Array<f32, 3> {
	material_specular
		+ (array![1.0; 3] - material_specular) * (1.0 - light_dir.dot(normal).powf(5.0))
}

#[allow(dead_code)]
fn attenuation(
	light_position: Vector<f32, 3>,
	position: Vector<f32, 3>,
	c: f32,
	l: f32,
	q: f32,
) -> Array<f32, 3> {
	let distance = (light_position - position).magnitude();
	array![1.0 / (c + l * distance + q * distance * distance); 3]
}

pub struct Test {}

impl Test {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();
		lights.iter().fold(material.emissive, |sum, light| {
			sum + Self::light(light, position, normal, material, camera_dir)
		}) * 255.0
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
		let specular = if diffuse >= 0.0 {
			phong(light_dir, camera_dir, normal).powf(material.shininess)
		} else {
			0.0
		};

		attenuation(light.position, position, 1.0, 0.0, 0.0)
			* (material.ambient * light.ambient
				+ material.diffuse * diffuse * light.diffuse
				+ schlick(material.specular, light_dir, normal) * specular * light.specular)
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
		let camera_dir = (camera - position).normalize();
		lights.iter().fold(material.emissive, |sum, light| {
			sum + Self::light(light, position, normal, material, camera_dir)
		}) * 255.0
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
			phong(light_dir, camera_dir, normal).powf(material.shininess)
		} else {
			0.0
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
		let camera_dir = (camera - position).normalize();
		lights.iter().fold(material.emissive, |sum, light| {
			sum + Self::light(light, position, normal, material, camera_dir)
		}) * 255.0
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
			blinn_phong(light_dir, camera_dir, normal).powf(material.shininess)
		} else {
			0.0
		};

		material.ambient * light.ambient
			+ material.diffuse * diffuse * light.diffuse
			+ material.specular * specular * light.specular
	}
}
