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
		ambience: Array<f32, 3>,
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
		ambience: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		match self {
			Model::Test => Test::reflect(position, normal, ambience, lights, material, camera),
			Model::Phong => Phong::reflect(position, normal, ambience, lights, material, camera),
			Model::BlinnPhong => {
				BlinnPhong::reflect(position, normal, ambience, lights, material, camera)
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
	// TODO: Highlights hopper fra kant til kant; phong beveger seg jevnt
	normal.dot((light_dir + camera_dir).normalize())
}

#[allow(dead_code)]
fn schlick_approximation(
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
	let dist = (light_position - position).magnitude();
	array![1.0 / (c + l * dist + q * dist * dist); 3]
}

pub struct Test {}

impl Test {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		ambience: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		lights.iter().fold(
			material.ambient_reflection * ambience + material.emissive_color,
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
		let specular = if diffuse >= 0.0 {
			phong_specular(light_dir, camera_dir, normal).powf(material.specular_exponent)
		} else {
			0.0
		};

		// TODO: Kikk på attenuation
		attenuation(light.position, position, 1.0, 0.0, 0.0)
			* (material.ambient_reflection * light.ambient_color
				+ material.diffuse_reflection * diffuse * light.diffuse_color
				// TODO: Dette gjør at vi får specular selv om den skal være null
				+ schlick_approximation(material.specular_reflection, light_dir, normal)
					* specular * light.specular_color)
	}
}

pub struct Phong {}

impl Phong {
	fn reflect(
		position: Vector<f32, 3>,
		normal: Vector<f32, 3>,
		ambience: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		lights.iter().fold(
			material.ambient_reflection * ambience + material.emissive_color,
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
			phong_specular(light_dir, camera_dir, normal).powf(material.specular_exponent)
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
		ambience: Array<f32, 3>,
		lights: &[Light],
		material: Material,
		camera: Vector<f32, 3>,
	) -> Array<f32, 3> {
		let camera_dir = (camera - position).normalize();

		lights.iter().fold(
			material.ambient_reflection * ambience + material.emissive_color,
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
			blinn_specular(light_dir, camera_dir, normal)
				// TODO
				.powf(material.specular_exponent * 4.0)
		} else {
			0.0
		};

		material.ambient_reflection * light.ambient_color
			+ material.diffuse_reflection * diffuse * light.diffuse_color
			+ material.specular_reflection * specular * light.specular_color
	}
}
