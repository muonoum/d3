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

use array::{array, Array};
use matrix::Vector;

#[derive(Default, Debug)]
pub struct Mesh {
	pub positions: Vec<Vector<f32, 3>>,
	pub normals: Vec<Vector<f32, 3>>,
	pub textures: Vec<Vector<f32, 2>>,
	pub materials: HashMap<String, Arc<Material>>,
	pub groups: Vec<Group>,
}

impl Mesh {
	pub fn new(path: &str) -> anyhow::Result<Mesh> {
		read_obj(path)
	}
}

#[derive(Debug, Clone)]
pub struct Group {
	pub name: String,
	pub material: Option<String>,
	pub faces: Vec<Face>,
}

impl Group {
	pub fn new(name: &str) -> Group {
		Group {
			name: name.into(),
			material: None,
			faces: Vec::new(),
		}
	}
}

#[derive(Debug, Clone)]
pub struct Material {
	pub name: String,

	pub normal_map: Option<image::RgbImage>,

	pub ambient: Array<f32, 3>,
	pub ambient_map: Option<image::RgbImage>,

	pub emissive: Array<f32, 3>,
	pub emissive_map: Option<image::RgbImage>,

	pub diffuse: Array<f32, 3>,
	pub diffuse_map: Option<image::RgbImage>,

	pub specular: Array<f32, 3>,
	pub specular_map: Option<image::RgbImage>,
	pub specular_exponent: f32,
}

impl Material {
	pub fn new(name: &str) -> Material {
		Material {
			name: name.into(),
			normal_map: None,
			ambient: array![0.0; 3],
			ambient_map: None,
			emissive: array![0.0; 3],
			emissive_map: None,
			diffuse: array![1.0; 3],
			diffuse_map: None,
			specular: array![0.0; 3],
			specular_map: None,
			specular_exponent: 1.0,
		}
	}

	pub fn ambient(&self, uv: Vector<f32, 2>) -> Array<f32, 3> {
		if let Some(ref texture) = self.ambient_map {
			self.ambient * Self::map(texture, uv)
		} else {
			self.diffuse
		}
	}

	pub fn try_ambient(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		uv.map(|uv| self.ambient(uv)).unwrap_or(self.ambient)
	}

	pub fn emissive(&self, uv: Vector<f32, 2>) -> Array<f32, 3> {
		if let Some(ref texture) = self.emissive_map {
			self.emissive * Self::map(texture, uv)
		} else {
			self.diffuse
		}
	}

	pub fn try_emissive(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		uv.map(|uv| self.emissive(uv)).unwrap_or(self.emissive)
	}

	pub fn diffuse(&self, uv: Vector<f32, 2>) -> Array<f32, 3> {
		if let Some(ref texture) = self.diffuse_map {
			self.diffuse * Self::map(texture, uv)
		} else {
			self.diffuse
		}
	}

	pub fn try_diffuse(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		uv.map(|uv| self.diffuse(uv)).unwrap_or(self.diffuse)
	}

	pub fn specular(&self, uv: Vector<f32, 2>) -> Array<f32, 3> {
		if let Some(ref texture) = self.specular_map {
			self.specular * Self::map(texture, uv)
		} else {
			self.diffuse
		}
	}

	pub fn try_specular(&self, uv: Option<Vector<f32, 2>>) -> Array<f32, 3> {
		uv.map(|uv| self.specular(uv)).unwrap_or(self.specular)
	}

	pub fn map(texture: &image::RgbImage, uv: Vector<f32, 2>) -> Array<f32, 3> {
		let width = texture.width() as f32;
		let height = texture.height() as f32;
		let x = (0.0f32).max(uv[0] * width).min(width - 1.0);
		let y = (0.0f32).max(uv[1] * height).min(height - 1.0);
		let rgb = texture.get_pixel(x as u32, y as u32);

		array![
			rgb[0] as f32 / 255.0,
			rgb[1] as f32 / 255.0,
			rgb[2] as f32 / 255.0
		]
	}
}

pub type Face = [Vertex; 3];

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: usize,
	pub normal: Option<usize>,
	pub tangent: Option<Vector<f32, 3>>,
	pub texture: Option<usize>,
}

fn read_obj(path: &str) -> anyhow::Result<Mesh> {
	let path = Path::new(path);
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut mesh = Mesh::default();
	let mut default_group = Group::new("default");
	let mut group: Option<Group> = None;

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
				if let Some(ref mut group) = group {
					group.faces.push(read_face(terms).context("f")?);
				} else {
					default_group.faces.push(read_face(terms).context("f")?);
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
			Some("vt") => mesh.textures.push(read_vector(terms).context("vt")?),
			Some(_) | None => {}
		}
	}

	if let Some(ref group) = group {
		mesh.groups.push(group.clone());
	}

	if !default_group.faces.is_empty() {
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

				Some("Ka") => mtl.ambient = read_array(terms).context("Ka")?,
				Some("Kd") => mtl.diffuse = read_array(terms).context("Kd")?,
				Some("Ke") => mtl.emissive = read_array(terms).context("Ke")?,
				Some("Ks") => mtl.specular = read_array(terms).context("Ks")?,

				Some("map_Ka") => {
					mtl.ambient_map = Some(read_map(terms, location).context("map_Ka")?)
				}

				Some("map_Kd") => {
					mtl.diffuse_map = Some(read_map(terms, location).context("map_Kd")?)
				}

				Some("map_Ke") => {
					mtl.emissive_map = Some(read_map(terms, location).context("map_Ke")?)
				}

				Some("map_Ks") => {
					mtl.specular_map = Some(read_map(terms, location).context("map_Ks")?)
				}

				Some("map_Bump") => {
					mtl.normal_map = Some(read_map(terms, location).context("map_Bump")?);
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

fn read_map(terms: SplitWhitespace, location: &Path) -> anyhow::Result<image::RgbImage> {
	Ok(image::open(read_path(terms, location)?)?.to_rgb8())
}

fn read_vector<const D: usize>(mut terms: SplitWhitespace) -> anyhow::Result<Vector<f32, D>> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("vector")?.parse()?);
	}

	Ok(Vector::new([cells.as_slice().try_into()?]))
}

fn read_array<const D: usize>(mut terms: SplitWhitespace) -> anyhow::Result<Array<f32, D>> {
	let mut cells = vec![];

	for _ in 0..D {
		cells.push(terms.next().context("array")?.parse()?);
	}

	Ok(Array::new(cells.as_slice().try_into()?))
}

fn read_face(mut terms: SplitWhitespace) -> anyhow::Result<Face> {
	Ok([
		read_vertex(terms.next().context("vertex")?)?,
		read_vertex(terms.next().context("vertex")?)?,
		read_vertex(terms.next().context("vertex")?)?,
	])
}

fn read_vertex(term: &str) -> Result<Vertex, anyhow::Error> {
	let terms = term.split("/").take(3).collect();
	let position = read_index(&terms, 0).context("position")?;
	let texture = read_index(&terms, 1);
	let normal = read_index(&terms, 2);

	Ok(Vertex {
		position: position - 1,
		normal: normal.map(|i| i - 1),
		texture: texture.map(|i| i - 1),
		tangent: None,
	})
}

fn read_index(terms: &Vec<&str>, i: usize) -> Option<usize> {
	terms.get(i).and_then(|v| v.parse::<usize>().ok())
}
