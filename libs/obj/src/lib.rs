use anyhow::Context;
use array::{array, Array};
use core::str::SplitAsciiWhitespace;
use matrix::Vector;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Mesh {
	pub faces: Vec<Face>,
	pub normals: Vec<Vector<f32, 3>>,
	pub positions: Vec<Vector<f32, 3>>,
	pub texture: Vec<Vector<f32, 2>>,
}

#[derive(Debug, Clone)]
pub struct Material {
	pub ambient_component: Array<f32, 3>,
	pub diffuse_component: Array<f32, 3>,
	pub emissive_component: Array<f32, 3>,
	pub specular_component: Array<f32, 3>,
	pub specular_exponent: f32,
	// pub diffuse_map: Option<String>,
}

impl Default for Material {
	fn default() -> Self {
		Self {
			ambient_component: array![0.0;  3],
			diffuse_component: array![1.0;  3],
			emissive_component: array![0.0;  3],
			specular_component: array![0.0;  3],
			specular_exponent: 0.0,
			// diffuse_map: None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct Face {
	pub vertices: [Vertex; 3],
	pub material: Option<Material>,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: usize,
	pub normal: usize,
	pub texture: Option<usize>,
}

impl Mesh {
	pub fn new(path: &str) -> anyhow::Result<Mesh> {
		let obj_path = Path::new(path);
		let obj_file = File::open(obj_path)?;
		let reader = BufReader::new(obj_file);

		let mut mesh = Mesh {
			positions: vec![],
			normals: vec![],
			faces: vec![],
			texture: vec![],
		};

		let mut current_material: String = "".into();
		let mut material_lib = HashMap::new();

		for line in reader.lines() {
			let line = line?;
			let mut terms = line.split_ascii_whitespace();

			match terms.next() {
				Some("mtllib") => {
					let location = obj_path.parent().context("mtllib location")?;
					let mtllib_path = Path::new(terms.next().context("mtllib path")?);
					read_materials(&location.join(mtllib_path), &mut material_lib)?;
				}

				Some("usemtl") => current_material = terms.next().context("usemtl")?.into(),
				Some("v") => mesh.positions.push(read_vector(terms).context("position")?),
				Some("vn") => mesh.normals.push(read_vector(terms).context("normal")?),
				Some("vt") => mesh.texture.push(read_vector(terms).context("texture")?),
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

fn read_vector<const D: usize>(
	mut terms: SplitAsciiWhitespace,
) -> Result<Vector<f32, D>, anyhow::Error> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("vector")?.parse()?);
	}

	Ok(Vector::new([cells.as_slice().try_into()?]))
}

fn read_array<const D: usize>(
	mut terms: SplitAsciiWhitespace,
) -> Result<Array<f32, D>, anyhow::Error> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("array")?.parse()?);
	}

	Ok(Array::new(cells.as_slice().try_into()?))
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
	let texture = read_index(&terms, 1); //.context("texture index")?;
	let normal = read_index(&terms, 2).context("normal index")?;

	Ok(Vertex {
		position: position - 1,
		normal: normal - 1,
		texture: texture.map(|i| i - 1),
	})
}

fn read_index(terms: &Vec<&str>, i: usize) -> Option<usize> {
	terms.get(i).and_then(|v| v.parse::<usize>().ok())
}

fn read_materials(path: &Path, lib: &mut HashMap<String, Material>) -> anyhow::Result<()> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let mut current_material: String = "".into();

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_ascii_whitespace();
		let term = terms.next();

		if let Some("newmtl") = term {
			let name = terms.next().context("newmtl")?;
			lib.insert(name.into(), Material::default());
			current_material = name.into();
		} else if let Some(v) = lib.get_mut(&current_material) {
			match term {
				Some("Ns") => v.specular_exponent = terms.next().context("Ns")?.parse::<f32>()?,
				Some("Ka") => v.ambient_component = read_array(terms)?,
				Some("Kd") => v.diffuse_component = read_array(terms)?,
				Some("Ks") => v.specular_component = read_array(terms)?,
				Some("Ke") => v.emissive_component = read_array(terms)?,
				// TODO
				// Some("map_Kd") => v.diffuse_map = Some(terms.next().context("diffuse map")?.into()),
				Some("#") | Some("Ni") | Some("d") | Some("illum") | Some("map_Kd") | None => {}
				Some(other) => anyhow::bail!("unexpected {}", other),
			}
		}
	}

	Ok(())
}
