use anyhow::Context;
use core::str::SplitAsciiWhitespace;
use matrix::vector::Vector;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Mesh {
	pub faces: Vec<Face>,
	pub normals: Vec<Vector<f32, 3>>,
	pub positions: Vec<Vector<f32, 3>>,
	// texture: Vec<Vector<f32, 2>>,
}

pub type Face = [Vertex; 3];

#[derive(Debug)]
pub struct Vertex {
	pub position: usize,
	pub normal: usize,
}

pub fn load(path: &str) -> anyhow::Result<Mesh> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let mut mesh = Mesh {
		positions: vec![],
		normals: vec![],
		faces: vec![],
	};

	for line in reader.lines() {
		let line = line?;
		let mut terms = line.split_ascii_whitespace();

		match terms.next() {
			Some("v") => mesh.positions.push(read_vector(terms)?),
			Some("vn") => mesh.normals.push(read_vector(terms)?),
			Some("f") => mesh.faces.push(read_face(terms)?),
			Some(_) => {}
			None => {}
		}
	}

	Ok(mesh)
}

fn read_vector(terms: SplitAsciiWhitespace) -> Result<Vector<f32, 3>, anyhow::Error> {
	let mut ts = terms
		.map(|t| t.parse())
		.take_while(Result::is_ok)
		.map(Result::unwrap);

	Ok(Vector::new([[
		ts.next().context("bad vector")?,
		ts.next().context("bad vector")?,
		ts.next().context("bad vector")?,
	]]))
}

fn read_face(mut terms: SplitAsciiWhitespace) -> Result<Face, anyhow::Error> {
	let v1 = read_vertex(terms.next().context("bad vertex")?)?;
	let v2 = read_vertex(terms.next().context("bad vertex")?)?;
	let v3 = read_vertex(terms.next().context("bad vertex")?)?;

	Ok([v1, v2, v3])
}

fn read_vertex(term: &str) -> Result<Vertex, anyhow::Error> {
	let terms: Vec<&str> = term.split("/").take(3).collect();
	let position = read_index(&terms, 0).context("bad position index")?;
	// let _texture = read_index(&terms, 1).context("bad texture index")?;
	let normal = read_index(&terms, 2).context("bad normal index")?;

	Ok(Vertex {
		position: position - 1,
		normal: normal - 1,
	})
}

fn read_index(terms: &Vec<&str>, i: usize) -> Option<usize> {
	terms.get(i).and_then(|v| v.parse::<usize>().ok())
}
