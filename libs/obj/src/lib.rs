#![feature(array_chunks)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(let_chains)]

use anyhow::Context;
use std::{
	collections::HashMap,
	fs::File,
	io::{BufRead, BufReader},
	path::{Path, PathBuf},
	str::SplitWhitespace,
	sync::Arc,
};

use array::{Array, array};
use matrix::Vector;

mod material;
pub use material::Material;

#[derive(Default, Debug)]
pub struct Mesh {
	pub positions: Vec<Vector<f32, 3>>,
	pub normals: Vec<Vector<f32, 3>>,
	pub uvs: Vec<Vector<f32, 2>>,
	pub vertices: Vec<Vertex>,
	pub materials: HashMap<String, Arc<Material>>,
	pub groups: Vec<Group>,
}

impl<'a> Mesh {
	pub fn triangles(&'a self) -> impl Iterator<Item = ([Vertex; 3], Option<&'a Arc<Material>>)> {
		std::iter::from_coroutine(
			#[coroutine]
			|| {
				for group in self.groups.iter() {
					let material = group
						.material
						.as_ref()
						.and_then(|name| self.materials.get(name));

					for [a, b, c] in group.vertices.array_chunks::<3>() {
						let vs = [self.vertices[*a], self.vertices[*b], self.vertices[*c]];
						yield (vs, material);
					}
				}
			},
		)
	}
}

#[derive(Debug, Clone)]
pub struct Group {
	pub name: String,
	pub material: Option<String>,
	pub vertices: Vec<usize>,
}

type Index = (usize, Option<usize>, Option<usize>);
pub type Face = [usize; 3];

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: usize,
	pub normal: Option<usize>,
	pub uv: Option<usize>,
}

impl Mesh {
	pub fn new(path: &str) -> anyhow::Result<Mesh> {
		read_obj(path)
	}
}

impl Group {
	pub fn new(name: &str) -> Group {
		Group {
			name: name.into(),
			material: None,
			vertices: Vec::new(),
		}
	}
}

fn read_obj(path: &str) -> anyhow::Result<Mesh> {
	let path = Path::new(path);
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut mesh = Mesh::default();
	let mut default_group = Group::new("default");
	let mut group: Option<Group> = None;

	let mut indices = HashMap::<Index, usize>::new();
	let mut add_vertex = |index| {
		indices.get(&index).cloned().unwrap_or_else(|| {
			mesh.vertices.push(Vertex {
				position: index.0,
				normal: index.1,
				uv: index.2,
			});

			let new_index = mesh.vertices.len() - 1;
			indices.insert(index, new_index);
			new_index
		})
	};

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_whitespace();

		match terms.next() {
			Some("mtllib") => {
				let location = path.parent().context("mtllib")?;
				read_materials(read_path(terms, location)?, location, &mut mesh.materials)?;
			}

			Some("g") => {
				if let Some(ref group) = group {
					mesh.groups.push(group.clone());
				}

				let name = terms.next().context("g")?;
				group = Some(Group::new(name));
			}

			Some("f") => {
				for index in read_face(terms).context("f")?.iter() {
					let vertex = add_vertex(*index);

					if let Some(ref mut group) = group {
						group.vertices.push(vertex);
					} else {
						default_group.vertices.push(vertex);
					}
				}
			}

			Some("usemtl") => {
				let name = terms.next().context("usemtl")?;

				if let Some(ref mut group) = group {
					group.material = Some(name.into());
				} else {
					default_group.material = Some(name.into());
				}
			}

			Some("v") => mesh.positions.push(read_vector(terms).context("v")?),
			Some("vn") => mesh.normals.push(read_vector(terms).context("vn")?),
			Some("vt") => mesh.uvs.push(read_vector(terms).context("vt")?),

			Some(_) | None => {}
		}
	}

	if let Some(ref group) = group {
		mesh.groups.push(group.clone());
	}

	if !default_group.vertices.is_empty() {
		mesh.groups.push(default_group);
	}

	Ok(mesh)
}

fn read_path(mut terms: SplitWhitespace, location: &Path) -> anyhow::Result<PathBuf> {
	let path: PathBuf = terms.next().context("path")?.into();

	Ok(if path.is_relative() {
		location.join(path)
	} else {
		path.to_path_buf()
	})
}

fn read_materials(
	path: PathBuf,
	location: &Path,
	lib: &mut HashMap<String, Arc<Material>>,
) -> anyhow::Result<()> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let mut material: Option<(String, Material)> = None;

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_whitespace();
		let term = terms.next();

		if let Some("newmtl") = term {
			if let Some((ref name, ref mtl)) = material {
				lib.insert(name.clone(), Arc::new(mtl.clone()));
			}

			let name = terms.next().context("newmtl")?;
			material = Some((name.into(), Material::new(name)))
		} else if let Some((_, ref mut mtl)) = material {
			match term {
				Some("Ns") => {
					mtl.specular_exponent =
						terms.next().context("Ns")?.parse::<f32>().context("Ns")?
				}

				Some("Ka") => mtl.ambient = read_color(terms).context("Ka")?,
				Some("Kd") => mtl.diffuse = read_color(terms).context("Kd")?,
				Some("Ke") => mtl.emissive = read_color(terms).context("Ke")?,
				Some("Ks") => mtl.specular = read_color(terms).context("Ks")?,

				Some("map_Ns") => {
					mtl.specular_exponent_map =
						Some(read_map(terms, location).context("map_Ns")?.to_luma8())
				}

				Some("map_Ka") => {
					mtl.ambient_map = Some(read_map(terms, location).context("map_Ka")?.to_rgb8())
				}

				Some("map_Kd") => {
					mtl.diffuse_map = Some(read_map(terms, location).context("map_Kd")?.to_rgb8())
				}

				Some("map_Ke") => {
					mtl.emissive_map = Some(read_map(terms, location).context("map_Ke")?.to_rgb8())
				}

				Some("map_Ks") => {
					mtl.specular_map = Some(read_map(terms, location).context("map_Ks")?.to_rgb8())
				}

				Some("map_Bump") => {
					mtl.normal_map = Some(read_map(terms, location).context("map_Bump")?.to_rgb8());
				}

				Some(_) | None => {}
			}
		}
	}

	if let Some((ref name, ref mtl)) = material {
		lib.insert(name.clone(), Arc::new(mtl.clone()));
	}

	Ok(())
}

fn read_map(terms: SplitWhitespace, location: &Path) -> anyhow::Result<image::DynamicImage> {
	let file = File::open(read_path(terms, location)?)?;
	let mut reader = image::ImageReader::new(BufReader::new(file)).with_guessed_format()?;
	reader.no_limits();
	Ok(reader.decode()?)
}

fn read_vector<const D: usize>(mut terms: SplitWhitespace) -> anyhow::Result<Vector<f32, D>> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("vector")?.parse()?);
	}

	Ok(Vector::new([cells.as_slice().try_into()?]))
}

fn read_color<const D: usize>(mut terms: SplitWhitespace) -> anyhow::Result<Array<f32, D>> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("array")?.parse()?);
	}

	Ok(Array::new(cells.as_slice().try_into()?))
}

fn read_face(terms: SplitWhitespace) -> anyhow::Result<Vec<Index>> {
	let mut vertices = Vec::new();

	for term in terms {
		vertices.push(read_vertex(term).context("vertex")?);
	}

	Ok(vertices)
}

fn read_vertex(term: &str) -> Result<Index, anyhow::Error> {
	let terms = term.split("/").take(3).collect();

	let position = read_index(&terms, 0).context("position")?;
	let uv = read_index(&terms, 1);
	let normal = read_index(&terms, 2);

	Ok((position - 1, normal.map(|i| i - 1), uv.map(|i| i - 1)))
}

fn read_index(terms: &Vec<&str>, i: usize) -> Option<usize> {
	terms.get(i).and_then(|v| v.parse::<usize>().ok())
}
