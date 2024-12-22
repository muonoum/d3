use anyhow::Context;
use array::{array, Array};
use core::str::SplitAsciiWhitespace;
use matrix::vector::Vector;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
		let mut material_lib = std::collections::HashMap::new();

		for line in reader.lines() {
			let line = line?;
			let mut terms = line.split_ascii_whitespace();

			match terms.next() {
				Some("mtllib") => {
					let parent = Path::new(path).parent().context("path")?;
					let lib = Path::new(terms.next().context("mtllib")?);
					let file = File::open(parent.join(lib))?;
					read_mtllib(file, &mut material_lib)?;
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

fn read_vector(terms: SplitAsciiWhitespace) -> Result<Vector<f32, 3>, anyhow::Error> {
	let mut ts = terms
		.map(|t| t.parse())
		.take_while(Result::is_ok)
		.map(Result::unwrap);

	Ok(Vector::new([[
		ts.next().context("vector")?,
		ts.next().context("vector")?,
		ts.next().context("vector")?,
	]]))
}

fn read_face(
	mut terms: SplitAsciiWhitespace,
	material: Option<Material>,
) -> Result<Face, anyhow::Error> {
	let v1 = read_vertex(terms.next().context("vertex")?)?;
	let v2 = read_vertex(terms.next().context("vertex")?)?;
	let v3 = read_vertex(terms.next().context("vertex")?)?;

	Ok(Face {
		vertices: [v1, v2, v3],
		material,
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

fn read_mtllib(
	file: std::fs::File,
	lib: &mut std::collections::HashMap<String, Material>,
) -> anyhow::Result<()> {
	let reader = BufReader::new(file);
	let mut reading_material: String = "".into();

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_ascii_whitespace();

		match terms.next() {
			Some("newmtl") => {
				let name = terms.next().context("newmtl")?;
				lib.insert(name.into(), Material::default());
				reading_material = name.into();
			}

			Some("Ns") => {
				let value = terms.next().context("Ns")?.parse::<f32>()?;
				lib.entry(reading_material.clone())
					.and_modify(|v| v.specular_exponent = value);
			}

			Some("Ka") => {
				let r = terms.next().context("Ka")?.parse::<f32>()?;
				let g = terms.next().context("Ka")?.parse::<f32>()?;
				let b = terms.next().context("Ka")?.parse::<f32>()?;
				lib.entry(reading_material.clone())
					.and_modify(|v| v.ambient_component = array![r, g, b]);
			}

			Some("Kd") => {
				let r = terms.next().context("Kd")?.parse::<f32>()?;
				let g = terms.next().context("Kd")?.parse::<f32>()?;
				let b = terms.next().context("Kd")?.parse::<f32>()?;
				lib.entry(reading_material.clone())
					.and_modify(|v| v.diffuse_component = array![r, g, b]);
			}

			Some("Ks") => {
				let r = terms.next().context("Ks")?.parse::<f32>()?;
				let g = terms.next().context("Ks")?.parse::<f32>()?;
				let b = terms.next().context("Ks")?.parse::<f32>()?;
				lib.entry(reading_material.clone())
					.and_modify(|v| v.specular_component = array![r, g, b]);
			}

			Some("Ke") => {
				let r = terms.next().context("Ks")?.parse::<f32>()?;
				let g = terms.next().context("Ks")?.parse::<f32>()?;
				let b = terms.next().context("Ks")?.parse::<f32>()?;
				lib.entry(reading_material.clone())
					.and_modify(|v| v.emissive_component = array![r, g, b]);
			}

			Some("#") | Some("Ni") | Some("d") | Some("illum") | None => {}
			Some(other) => anyhow::bail!("unexpected {}", other),
		}
	}

	Ok(())
}
