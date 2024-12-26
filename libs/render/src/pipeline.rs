use crate::Buffer;
use crate::Interpolate;
use matrix::Vector;
use matrix::vector;

pub trait Pipeline {
	type Setup;
	type Face;
	type Vertex;
	type Attributes;
	type Fragment;

	fn setup(&self) -> Self::Setup;
	fn face(&self, face: &Self::Face) -> [Self::Vertex; 3];

	fn vertex(
		&self,
		vertex: &Self::Vertex,
		setup: &Self::Setup,
	) -> (Vector<f32, 4>, Self::Attributes);

	fn fragment(&self, face: &Self::Face, data: &Self::Attributes) -> Self::Fragment;
}

pub fn screen_space(ndc: Vector<f32, 3>, width: f32, height: f32) -> Vector<f32, 3> {
	Vector::new([[
		(ndc[0] + 1.0) / 2.0 * width,
		(1.0 - ndc[1]) / 2.0 * height,
		-ndc[2],
	]])
}

pub fn render<V, F, P, A>(
	pipeline: impl Pipeline<Vertex = V, Face = F, Fragment = P, Attributes = A>,
	mesh: &[F],
	frame: &mut impl Buffer<P>,
	depth: &mut [f32],
) where
	A: Copy + Interpolate,
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

	let setup = pipeline.setup();

	for face in mesh.iter() {
		let [v1, v2, v3] = pipeline.face(face);

		let (clip1, data1) = pipeline.vertex(&v1, &setup);
		let (clip2, data2) = pipeline.vertex(&v2, &setup);
		let (clip3, data3) = pipeline.vertex(&v3, &setup);

		// TODO: Actual clipping
		if clipped(clip1) || clipped(clip2) || clipped(clip3) {
			continue;
		}

		let screen1 = screen_space(clip1.v3());
		let screen2 = screen_space(clip2.v3());
		let screen3 = screen_space(clip3.v3());

		let normal = Vector::cross(screen2 - screen1, screen3 - screen1);
		if normal[2] > 0.0 {
			continue;
		}

		let rz1 = 1.0 / -clip1[3];
		let rz2 = 1.0 / -clip2[3];
		let rz3 = 1.0 / -clip3[3];

		let data1 = Interpolate::scale(data1, rz1);
		let data2 = Interpolate::scale(data2, rz2);
		let data3 = Interpolate::scale(data3, rz3);

		rasterize(screen1, screen2, screen3, width, height, |x, y, u, v, w| {
			let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

			let i = y * width + x;
			if depth[i] < z {
				let data = Interpolate::barycentric(data1, u, data2, v, data3, w);
				let data = Interpolate::scale(data, z);
				frame.put(x, y, pipeline.fragment(face, &data));
				depth[i] = z;
			}
		});
	}
}

pub fn clipped(v: Vector<f32, 4>) -> bool {
	let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
	let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
	let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
	x || y || z
}

pub fn rasterize(
	p1: Vector<f32, 3>,
	p2: Vector<f32, 3>,
	p3: Vector<f32, 3>,
	width: usize,
	height: usize,
	mut fragment: impl FnMut(usize, usize, f32, f32, f32),
) {
	let (min_x, min_y, max_x, max_y) = bounding_box((p1, p2, p3), (width, height));
	let area = 1.0 / edge(p1.into(), p2.into(), p3.into());
	let point = vector![min_x as f32, min_y as f32];

	let mut r1 = edge(p2.into(), p3.into(), point);
	let mut r2 = edge(p3.into(), p1.into(), point);
	let mut r3 = edge(p1.into(), p2.into(), point);

	for y in min_y..=max_y {
		let mut u = r1;
		let mut v = r2;
		let mut w = r3;

		for x in min_x..=max_x {
			if u >= 0.0 && v >= 0.0 && w >= 0.0 {
				let u = u * area;
				let v = v * area;
				let w = w * area;

				fragment(x, y, u, v, w);
			}

			u += p3[1] - p2[1];
			v += p1[1] - p3[1];
			w += p2[1] - p1[1];
		}

		r1 += p2[0] - p3[0];
		r2 += p3[0] - p1[0];
		r3 += p1[0] - p2[0];
	}
}

pub fn bounding_box(
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
	let max_x = x1.max(x2.max(x3)).min(width - 1);
	let max_y = y1.max(y2.max(y3)).min(height - 1);

	(min_x, min_y, max_x, max_y)
}

pub fn edge<T: matrix::Cell>(a: Vector<T, 2>, b: Vector<T, 2>, p: Vector<T, 2>) -> T {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
}
