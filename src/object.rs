use crate::material::Material;
use crate::matrix::vector::Vector;
use crate::mesh;
use crate::mesh::Mesh;
use crate::vector;

pub struct Object {
	pub scale: Vector<f32, 3>,
	pub orientation: Vector<f32, 3>,
	pub position: Vector<f32, 3>,
	pub material: Material,
	pub mesh: Mesh,
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

		let object = Object {
			mesh: mesh::load(path)?,
			scale: vector![1.0; 3],
			orientation: vector![0.0; 3],
			position: vector![0.0; 3],
			material,
		};

		Ok(object)
	}
}
