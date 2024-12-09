use anyhow::Context;
use matrix::vector::Vector;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Face {
    pub vertices: [Vertex; 3],
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vector<f64, 3>,
    pub normal: Option<Vector<f64, 3>>,
}

pub fn load(path: &str) -> anyhow::Result<Vec<Face>> {
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
                    let x: Vec<&str> = term.split("/").take(3).collect();
                    let v = x.get(0).and_then(|v| v.parse::<usize>().ok());
                    let t = x.get(1).and_then(|v| v.parse::<usize>().ok());
                    let n = x.get(2).and_then(|v| v.parse::<usize>().ok());
                    f.push((v, t, n));
                }

                fs.push(f.try_into().unwrap());
            }

            Some(_) => {}
            None => {}
        }
    }

    let mut faces = vec![];

    for f in fs {
        let [(v1, _, vn1), (v2, _, vn2), (v3, _, vn3)] = f;

        faces.push(Face {
            vertices: [
                Vertex {
                    position: vs[v1.unwrap() - 1],
                    normal: vn1.map(|vn| vns[vn - 1]),
                },
                Vertex {
                    position: vs[v2.unwrap() - 1],
                    normal: vn2.map(|vn| vns[vn - 1]),
                },
                Vertex {
                    position: vs[v3.unwrap() - 1],
                    normal: vn3.map(|vn| vns[vn - 1]),
                },
            ],
        });
    }

    Ok(faces)
}
