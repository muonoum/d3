use crate::{
	buffer::{Target, Texture},
	varying::Varying,
};
use matrix::{Vector, vector};

pub trait Pipeline {
	type Vertex;
	type Varying: Varying;
	type Fragment;

	fn vertex(&self, vertex: Self::Vertex) -> (Vector<f32, 4>, Self::Varying);
	fn fragment(&self, varying: Self::Varying) -> Self::Fragment;
}

pub fn screen_space(ndc: Vector<f32, 3>, width: f32, height: f32) -> Vector<f32, 3> {
	vector![
		(ndc[0] + 1.0) / 2.0 * width,
		(1.0 - ndc[1]) / 2.0 * height,
		-ndc[2],
	]
}

pub fn run<V, F>(
	pipeline: impl Pipeline<Vertex = V, Fragment = F>,
	vertices: impl Iterator<Item = V>,
	mut fragment_buffer: impl Target<Unit = F>,
	mut depth_buffer: impl Target<Unit = f32> + Texture<Unit = f32>,
) {
	let width = fragment_buffer.width();
	let height = fragment_buffer.height();
	assert_eq!(width, depth_buffer.width());
	assert_eq!(height, depth_buffer.height());

	let screen_space = |v| screen_space(v, width as f32, height as f32);

	for [a, b, c] in vertices.array_chunks::<3>() {
		let (clip1, var1) = pipeline.vertex(a);
		let (clip2, var2) = pipeline.vertex(b);
		let (clip3, var3) = pipeline.vertex(c);

		let screen1 = screen_space(clip1.v3());
		let screen2 = screen_space(clip2.v3());
		let screen3 = screen_space(clip3.v3());

		let normal = (screen2 - screen1).cross(screen3 - screen1);
		if normal[2] > 0.0 {
			continue;
		}

		let rz1 = 1.0 / -clip1[3];
		let rz2 = 1.0 / -clip2[3];
		let rz3 = 1.0 / -clip3[3];

		let var1 = var1.scale(rz1);
		let var2 = var2.scale(rz2);
		let var3 = var3.scale(rz3);

		render_triangle(screen1, screen2, screen3, width, height, |x, y, u, v, w| {
			let z = 1.0 / (u * rz1 + v * rz2 + w * rz3);

			if depth_buffer.get(x, y) < z {
				depth_buffer.put(x, y, z);
			} else {
				return;
			}

			let varying = Varying::barycentric(var1, u, var2, v, var3, w).scale(z);
			let fragment = pipeline.fragment(varying);
			fragment_buffer.put(x, y, fragment);
		})
	}
}

pub fn edge(a: Vector<f32, 3>, b: Vector<f32, 3>, p: Vector<f32, 3>) -> f32 {
	(p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
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

pub fn render_triangle(
	p1: Vector<f32, 3>,
	p2: Vector<f32, 3>,
	p3: Vector<f32, 3>,
	width: usize,
	height: usize,
	mut pixel: impl FnMut(usize, usize, f32, f32, f32),
) {
	let (min_x, min_y, max_x, max_y) = bounding_box((p1, p2, p3), (width, height));
	let area = 1.0 / edge(p1, p2, p3);
	let point = vector![min_x as f32, min_y as f32, 0.0];

	let mut r1 = edge(p2, p3, point);
	let mut r2 = edge(p3, p1, point);
	let mut r3 = edge(p1, p2, point);

	for y in min_y..=max_y {
		let mut u = r1;
		let mut v = r2;
		let mut w = r3;

		for x in min_x..=max_x {
			if u >= 0.0 && v >= 0.0 && w >= 0.0 {
				let u = u * area;
				let v = v * area;
				let w = w * area;

				pixel(x, y, u, v, w);
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
