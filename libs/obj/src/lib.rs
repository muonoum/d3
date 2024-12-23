use anyhow::Context;
use array::{array, Array};
use core::str::SplitAsciiWhitespace;
use matrix::{vector, Vector};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Mesh {
	pub faces: Vec<Face>,
	pub normals: Vec<Vector<f32, 3>>,
	pub positions: Vec<Vector<f32, 3>>,
}

#[derive(Debug, Copy, Clone)]
pub struct Material {
	pub ambient_component: Array<f32, 3>,
	pub diffuse_component: Array<f32, 3>,
	pub emissive_component: Array<f32, 3>,
	pub specular_component: Array<f32, 3>,
	pub specular_exponent: f32,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			ambient_component: array![0.0;  3],
			diffuse_component: array![1.0;  3],
			emissive_component: array![0.0;  3],
			specular_component: array![0.0;  3],
			specular_exponent: 0.0,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct Face {
	pub vertices: [Vertex; 3],
	pub material: Option<Material>,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: usize,
	pub normal: usize,
}

impl Mesh {
	pub fn new(path: &str) -> anyhow::Result<Mesh> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);

		let mut mesh = Mesh {
			positions: vec![],
			normals: vec![],
			faces: vec![],
		};

		let mut current_material: String = "".into();
		let mut material_lib = HashMap::new();

		for line in reader.lines() {
			let line = line?;
			let mut terms = line.split_ascii_whitespace();

			match terms.next() {
				Some("mtllib") => {
					let parent = Path::new(path).parent().context("path")?;
					let lib = Path::new(terms.next().context("mtllib")?);
					let file = File::open(parent.join(lib))?;
					read_materials(file, &mut material_lib)?;
				}

				Some("usemtl") => {
					let name = terms.next().context("usemtl")?;
					current_material = name.into();
				}

				Some("vt") => {}
				Some("v") => mesh.positions.push(read_vector(terms).context("position")?),
				Some("vn") => mesh.normals.push(read_vector(terms).context("normal")?),

				Some("f") => mesh.faces.push(
					read_face(terms, material_lib.get(&current_material).cloned())
						.context("face")?,
				),

				Some("#") | Some("####") | Some("o") | Some("s") | None => {}
				Some(other) => anyhow::bail!("unexpected {}", other),
			}
		}

		Ok(mesh)
	}
}

fn read_vector(mut terms: SplitAsciiWhitespace) -> Result<Vector<f32, 3>, anyhow::Error> {
	Ok(vector![
		terms.next().context("vector")?.parse()?,
		terms.next().context("vector")?.parse()?,
		terms.next().context("vector")?.parse()?,
	])
}

fn read_array(mut terms: SplitAsciiWhitespace) -> Result<Array<f32, 3>, anyhow::Error> {
	Ok(array![
		terms.next().context("array")?.parse()?,
		terms.next().context("array")?.parse()?,
		terms.next().context("array")?.parse()?,
	])
}

fn read_face(
	mut terms: SplitAsciiWhitespace,
	material: Option<Material>,
) -> Result<Face, anyhow::Error> {
	Ok(Face {
		material,
		vertices: [
			read_vertex(terms.next().context("vertex")?)?,
			read_vertex(terms.next().context("vertex")?)?,
			read_vertex(terms.next().context("vertex")?)?,
		],
	})
}

fn read_vertex(term: &str) -> Result<Vertex, anyhow::Error> {
	let terms: Vec<&str> = term.split("/").take(3).collect();
	let position = read_index(&terms, 0).context("position index")?;
	let normal = read_index(&terms, 2).context("normal index")?;

	Ok(Vertex {
		position: position - 1,
		normal: normal - 1,
	})
}

fn read_index(terms: &Vec<&str>, i: usize) -> Option<usize> {
	terms.get(i).and_then(|v| v.parse::<usize>().ok())
}

fn read_materials(file: File, lib: &mut HashMap<String, Material>) -> anyhow::Result<()> {
	let reader = BufReader::new(file);
	let mut current_material: String = "".into();

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_ascii_whitespace();

		match terms.next() {
			Some("newmtl") => {
				let name = terms.next().context("newmtl")?;
				lib.insert(name.into(), Material::default());
				current_material = name.into();
			}

			Some("Ns") => {
				if let Some(v) = lib.get_mut(&current_material) {
					v.specular_exponent = terms.next().context("Ns")?.parse::<f32>()?
				}
			}

			Some("Ka") => {
				if let Some(v) = lib.get_mut(&current_material) {
					v.ambient_component = read_array(terms)?;
				}
			}

			Some("Kd") => {
				if let Some(v) = lib.get_mut(&current_material) {
					v.diffuse_component = read_array(terms)?;
				}
			}

			Some("Ks") => {
				if let Some(v) = lib.get_mut(&current_material) {
					v.specular_component = read_array(terms)?;
				}
			}

			Some("Ke") => {
				if let Some(v) = lib.get_mut(&current_material) {
					v.emissive_component = read_array(terms)?;
				}
			}

			Some("#") | Some("Ni") | Some("d") | Some("illum") | None => {}
			Some(other) => anyhow::bail!("unexpected {}", other),
		}
	}

	Ok(())
}
