use crate::light::Light;
use crate::material::Material;
use array::array;
use array::Array;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Phong1,
	Phong2,
}

pub trait Reflect {
	fn reflect(
		&self,
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		ambient: Array<f32, 3>,
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
		ambient: Array<f32, 3>,
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		match self {
			Model::Phong1 => Phong1::reflect(position, normal, lights, ambient, material, camera),
			Model::Phong2 => Phong2::reflect(position, normal, lights, ambient, material, camera),
		}
	}
}

pub struct Phong1 {}

impl Phong1 {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		ambient: Array<f32, 3>,
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let color = material.emissive + ambient * material.ambient;
		lights.iter().fold(color.clamp(0.0, 1.0), |sum, l| {
			sum + Self::light(l, position, normal, material, camera)
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

		light.diffuse * material.diffuse * diffuse + light.specular * material.specular * specular
	}
}

pub struct Phong2 {}

impl Phong2 {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		lights: &[Light],
		ambient: Array<f32, 3>,
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		let (diffuse, specular) = lights.iter().fold(
			(array![0.0;3], array![0.0;3]),
			|(diffuse, specular), light| {
				let light_dir = (light.position - position).normalize();
				let d = normal.dot(light_dir).clamp(0.0, 1.0);
				let s = normal
					.dot((camera_dir + light_dir).normalize())
					.clamp(0.0, 1.0)
					.powf(material.shininess);
				(diffuse + light.diffuse * d, specular + light.specular * s)
			},
		);

		let sum = material.emissive
			+ ambient * material.ambient
			+ diffuse * material.diffuse
			+ specular * material.specular;
		sum.clamp(0.0, 1.0) * 255.0
	}
}
