use crate::Interpolate;
use crate::buffer::Buffer;
use matrix::Vector;
use matrix::vector;

pub trait Pipeline {
	type Face;
	type Vertex;
	type Varying;
	type Fragment;

	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3];
	fn vertex(&self, face: &Self::Face, vertex: Self::Vertex) -> (Vector<f32, 4>, Self::Varying);
	fn fragment(&self, face: &Self::Face, data: Self::Varying) -> Self::Fragment;
}

fn bounding_box(
	(p1, p2, p3): (Vector<f32, 3>, Vector<f32, 3>, Vector<f32, 3>),
	(width, height): (usize, usize),
) -> (usize, usize, usize, usize) {
	let x1 = p1[0] as usize;
	let y1 = p1[1] as usize;
	let x2 = p2[0] as usize;
	let y2 = p2[1] as usize;
	let x3 = p3[0] as usize;
	let y3 = p3[1] as usize;

	let min_x = x1.min(x2.min(x3)).max(0);
	let min_y = y1.min(y2.min(y3)).max(0);
	let max_x = x1.max(x2.max(x3)).min(width);
	let max_y = y1.max(y2.max(y3)).min(height);

	(min_x, min_y, max_x, max_y)
}

fn edge<T: matrix::matrix::Cell>(a: Vector<T, 2>, b: Vector<T, 2>, p: Vector<T, 2>) -> T {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}

fn clipped(v: Vector<f32, 4>) -> bool {
	let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
	let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
	let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
	x || y || z
}

pub fn render<V, F, P, D>(
	pipeline: impl Pipeline<Vertex = V, Face = F, Fragment = P, Varying = D>,
	mesh: &[F],
	frame: &mut impl Buffer<P>,
	depth: &mut [f32],
) where
	D: Copy + Interpolate,
{
	let height = frame.height();
	let width = frame.width();

	let screen_space = |ndc: Vector<f32, 3>| {
		Vector::new([[
			(ndc[0] + 1.0) / 2.0 * width as f32,
			(1.0 - ndc[1]) / 2.0 * height as f32,
			-ndc[2],
		]])
	};

	for face in mesh.iter() {
		let [v1, v2, v3] = pipeline.face(face);

		let (clip1, data1) = pipeline.vertex(face, v1);
		let ndc1 = clip1.v3();
		let screen1 = screen_space(ndc1);

		let (clip2, data2) = pipeline.vertex(face, v2);
		let ndc2 = clip2.v3();
		let screen2 = screen_space(ndc2);

		let (clip3, data3) = pipeline.vertex(face, v3);
		let ndc3 = clip3.v3();
		let screen3 = screen_space(ndc3);

		let normal = Vector::cross(screen2 - screen1, screen3 - screen1);
		if normal[2] > 0.0 {
			continue;
		}

		// TODO: Actual clipping
		if clipped(clip1) || clipped(clip2) || clipped(clip3) {
			continue;
		}

		let rz1 = 1.0 / -clip1[3];
		let rz2 = 1.0 / -clip2[3];
		let rz3 = 1.0 / -clip3[3];

		let data1 = Interpolate::perspective(data1, rz1);
		let data2 = Interpolate::perspective(data2, rz2);
		let data3 = Interpolate::perspective(data3, rz3);

		let (min_x, min_y, max_x, max_y) =
			bounding_box((screen1, screen2, screen3), (width - 1, height - 1));
		let area = 1.0 / edge(screen1.into(), screen2.into(), screen3.into());
		let point = vector![min_x as f32, min_y as f32];

		let mut r1 = edge(screen2.into(), screen3.into(), point);
		let mut r2 = edge(screen3.into(), screen1.into(), point);
		let mut r3 = edge(screen1.into(), screen2.into(), point);

		for y in min_y..=max_y {
			let mut u = r1;
			let mut v = r2;
			let mut w = r3;

			for x in min_x..=max_x {
				if u >= 0.0 && v >= 0.0 && w >= 0.0 {
					let u = u * area;
					let v = v * area;
					let w = w * area;

					let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

					let i = y * width + x;
					if depth[i] < z {
						let data = Interpolate::interpolate(data1, data2, data3, u, v, w);
						let data = Interpolate::perspective(data, z);
						frame.put(x, y, pipeline.fragment(face, data));
						depth[i] = z;
					}
				}

				u += screen3[1] - screen2[1];
				v += screen1[1] - screen3[1];
				w += screen2[1] - screen1[1];
			}

			r1 += screen2[0] - screen3[0];
			r2 += screen3[0] - screen1[0];
			r3 += screen1[0] - screen2[0];
		}
	}
}
