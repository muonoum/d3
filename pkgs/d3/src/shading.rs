use crate::light::Light;
use crate::material::Material;
use crate::reflection;
use crate::reflection::Reflect;
use matrix::vector;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Flat,
	Gourad,
	Phong,
}

pub trait Shade<'a> {
	#[allow(clippy::too_many_arguments)]
	fn shade(
		&'a self,
		reflection: &'a reflection::Model,
		positions: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		normals: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		camera: Vector<f32, 3>,
		lights: &'a [Light],
		material: Material,
	) -> Box<dyn Fn(f32, f32, f32) -> [u8; 4] + 'a>;
}

impl<'a> Shade<'a> for Model {
	fn shade(
		&'a self,
		reflection: &'a reflection::Model,
		positions: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		normals: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		camera: Vector<f32, 3>,
		lights: &'a [Light],
		material: Material,
	) -> Box<dyn Fn(f32, f32, f32) -> [u8; 4] + 'a> {
		match self {
			Model::Flat => {
				let (position1, position2, position3) = positions;
				let (normal1, normal2, normal3) = normals;

				let position = (position1 + position2 + position3) / 3.0;
				let normal = (normal1 + normal2 + normal3) / 3.0;

				let color = reflection.reflect(position, normal, lights, material, camera);

				Box::new(move |_: f32, _: f32, _: f32| {
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}

			Model::Gourad => {
				let get_color = |p, n| reflection.reflect(p, n, lights, material, camera);

				let (position1, position2, position3) = positions;
				let (normal1, normal2, normal3) = normals;

				let color1 = get_color(position1, normal1);
				let color2 = get_color(position2, normal2);
				let color3 = get_color(position3, normal3);

				Box::new(move |u: f32, v: f32, w: f32| {
					[
						(color1[0] * u + color2[0] * v + color3[0] * w) as u8,
						(color1[1] * u + color2[1] * v + color3[1] * w) as u8,
						(color1[2] * u + color2[2] * v + color3[2] * w) as u8,
						255,
					]
				})
			}

			Model::Phong => {
				let (position1, position2, position3) = positions;
				let (normal1, normal2, normal3) = normals;

				let get_color = move |p, n| reflection.reflect(p, n, lights, material, camera);

				Box::new(move |u: f32, v: f32, w: f32| {
					let position = vector![
						position1[0] * u + position2[0] * v + position3[0] * w,
						position1[1] * u + position2[1] * v + position3[1] * w,
						position1[2] * u + position2[2] * v + position3[2] * w,
					];

					let normal = vector![
						normal1[0] * u + normal2[0] * v + normal3[0] * w,
						normal1[1] * u + normal2[1] * v + normal3[1] * w,
						normal1[2] * u + normal2[2] * v + normal3[2] * w,
					];

					let color = get_color(position, normal);
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}
		}
	}
}
