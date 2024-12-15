use crate::material::Material;
use crate::mesh::Mesh;
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
