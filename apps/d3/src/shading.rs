use crate::light::Light;
use crate::material::Material;
use crate::reflection;
use crate::reflection::Reflect;
use array::Array;
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
		ambient_color: Array<f32, 3>,
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
		ambient_color: Array<f32, 3>,
		lights: &'a [Light],
		material: Material,
	) -> Box<dyn Fn(f32, f32, f32) -> [u8; 4] + 'a> {
		match self {
			Model::Flat => {
				let (position1, position2, position3) = positions;
				let position = (position1 + position2 + position3) / 3.0;
				let (normal1, normal2, normal3) = normals;
				let normal = ((normal1 + normal2 + normal3) / 3.0).normalize();

				let color =
					reflection.reflect(position, normal, ambient_color, lights, material, camera);

				Box::new(move |_: f32, _: f32, _: f32| {
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}

			Model::Gourad => {
				let get_color =
					|p, n| reflection.reflect(p, n, ambient_color, lights, material, camera);

				let (position1, position2, position3) = positions;
				let (normal1, normal2, normal3) = normals;

				let color1 = get_color(position1, normal1.normalize());
				let color2 = get_color(position2, normal2.normalize());
				let color3 = get_color(position3, normal3.normalize());

				Box::new(move |u: f32, v: f32, w: f32| {
					let color = color1 * u + color2 * v + color3 * w;
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}

			Model::Phong => {
				let (position1, position2, position3) = positions;
				let (normal1, normal2, normal3) = normals;

				let get_color =
					move |p, n| reflection.reflect(p, n, ambient_color, lights, material, camera);

				Box::new(move |u: f32, v: f32, w: f32| {
					let position = position1 * u + position2 * v + position3 * w;
					let normal = normal1 * u + normal2 * v + normal3 * w;
					let color = get_color(position, normal.normalize());
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}
		}
	}
}
