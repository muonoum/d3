use crate::light::Light;
use crate::material::Material;
use crate::reflection;
use array::Array;
use matrix::vector::Vector;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum Model {
	Flat,
	Gourad,
	Phong,
}

impl<'a> Model {
	#[allow(clippy::too_many_arguments)]
	pub fn shade(
		&'a self,
		reflection: &'a reflection::Model,
		screen: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		positions: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		normals: (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
		camera: Vector<f32, 3>,
		ambient_color: Array<f32, 3>,
		lights: &'a [Light],
		material: Material,
	) -> Box<dyn Fn(f32, f32, f32, f32) -> [u8; 4] + 'a> {
		match self {
			Model::Flat => {
				let (position1, position2, position3) = positions;
				let position = (position1 + position2 + position3) / 3.0;
				let (normal1, normal2, normal3) = normals;
				let normal = ((normal1 + normal2 + normal3) / 3.0).normalize();

				let color =
					reflection.reflect(position, normal, ambient_color, lights, material, camera);

				Box::new(move |_: f32, _: f32, _: f32, _: f32| {
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}

			Model::Gourad => {
				let get_color =
					|p, n| reflection.reflect(p, n, ambient_color, lights, material, camera);

				let color1 = get_color(positions.0, normals.0.normalize()) / screen.0[2];
				let color2 = get_color(positions.1, normals.1.normalize()) / screen.1[2];
				let color3 = get_color(positions.2, normals.2.normalize()) / screen.2[2];

				Box::new(move |u: f32, v: f32, w: f32, z: f32| {
					let color = (color1 * u + color2 * v + color3 * w) * z;
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}

			Model::Phong => {
				let get_color =
					move |p, n| reflection.reflect(p, n, ambient_color, lights, material, camera);

				let position1 = positions.0 / screen.0[2];
				let position2 = positions.1 / screen.1[2];
				let position3 = positions.2 / screen.2[2];
				let normal1 = normals.0 / screen.0[2];
				let normal2 = normals.1 / screen.1[2];
				let normal3 = normals.2 / screen.2[2];

				Box::new(move |u: f32, v: f32, w: f32, z: f32| {
					let position = (position1 * u + position2 * v + position3 * w) * z;
					let normal = (normal1 * u + normal2 * v + normal3 * w) * z;
					let color = get_color(position, normal.normalize());
					[color[0] as u8, color[1] as u8, color[2] as u8, 255]
				})
			}
		}
	}
}
