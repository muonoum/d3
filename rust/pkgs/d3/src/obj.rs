use anyhow::Context;
use matrix::vector::Vector;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Mesh<const D: usize> = Vec<Face<D>>;

pub struct Face<const D: usize> {
    pub vertices: [Vertex<D>; 3],
    pub normal: Vector<f64, 3>,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex<const D: usize> {
    pub position: Vector<f64, D>,
    pub normal: Vector<f64, D>,
}

pub fn load(path: &str) -> anyhow::Result<Mesh<3>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut vs = vec![];
    let mut vns = vec![];
    let mut fs = vec![];

    for line in reader.lines() {
        let line = line?;
        let mut terms = line.split_ascii_whitespace();

        match terms.next() {
            Some("v") => {
                let mut ts = terms
                    .map(|t| t.parse())
                    .take_while(Result::is_ok)
                    .map(Result::unwrap);

                vs.push(Vector::new([[
                    ts.next().context("bad vertex")?,
                    ts.next().context("bad vertex")?,
                    ts.next().context("bad vertex")?,
                ]]))
            }

            Some("vn") => {
                let mut ts = terms
                    .map(|t| t.parse())
                    .take_while(Result::is_ok)
                    .map(Result::unwrap);

                vns.push(Vector::new([[
                    ts.next().context("bad normal")?,
                    ts.next().context("bad normal")?,
                    ts.next().context("bad normal")?,
                ]]))
            }

            Some("f") => {
                let mut f = vec![];

                for term in terms {
                    let vertex: Vec<&str> = term.split("/").take(3).collect();
                    let v = vertex.get(0).and_then(|v| v.parse::<usize>().ok());
                    let vt = vertex.get(1).and_then(|v| v.parse::<usize>().ok());
                    let vn = vertex.get(2).and_then(|v| v.parse::<usize>().ok());
                    f.push((v, vt, vn));
                }

                fs.push(f.try_into().unwrap());
            }

            Some(_) => {}
            None => {}
        }
    }

    let mut faces = vec![];

    for f in fs {
        let [v1, v2, v3] = f;

        let v1 = {
            let (v1, _vt1, vn1) = v1;
            let position = vs[v1.context("missing vertex")? - 1];
            let normal = vns[vn1.context("missing normal")? - 1];
            Vertex { position, normal }
        };

        let v2 = {
            let (v2, _vt2, vn2) = v2;
            let position = vs[v2.context("missing vertex")? - 1];
            let normal = vns[vn2.context("missing normal")? - 1];
            Vertex { position, normal }
        };

        let v3 = {
            let (v3, _vt3, vn3) = v3;
            let position = vs[v3.context("missing vertex")? - 1];
            let normal = vns[vn3.context("missing normal")? - 1];
            Vertex { position, normal }
        };

        let normal = Vector::normalize({
            let a = v2.position - v1.position;
            let b = v3.position - v1.position;
            a.cross_product(b)
        });

        faces.push(Face {
            vertices: [v1, v2, v3],
            normal,
        });
    }

    Ok(faces)
}
