use crate::material::Material;
use crate::mesh;
use crate::mesh::Mesh;
use matrix::vector;
use matrix::vector::Vector;

#[derive(Debug)]
pub struct Object {
	pub material: Material,
	pub mesh: Mesh,
	pub orientation: Vector<f32, 3>,
	pub position: Vector<f32, 3>,
	pub scale: Vector<f32, 3>,
	pub update: Update,
}

#[derive(Debug)]
pub struct Update {
	pub orientation: Vector<f32, 3>,
}

impl Object {
	pub fn new(path: &str, material: Material) -> Result<Object, anyhow::Error> {
		let mesh = mesh::load(path)?;

		println!(
			"faces={} positions={} normals={}",
			mesh.faces.len(),
			mesh.positions.len(),
			mesh.normals.len(),
		);

		let update = Update {
			orientation: vector![0.0, 0.008, 0.0],
		};

		let object = Object {
			material,
			mesh: mesh::load(path)?,
			orientation: vector![0.0; 3],
			position: vector![0.0; 3],
			scale: vector![1.0; 3],
			update,
		};

		Ok(object)
	}
}
