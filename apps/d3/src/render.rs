use array::{array, Array};
use matrix::{vector, Vector};

use crate::light::Light;

pub fn clipped(v: Vector<f32, 4>) -> bool {
	let x = v[0] + v[3] < 0.0 || -v[0] + v[3] < 0.0;
	let y = v[1] + v[3] < 0.0 || -v[1] + v[3] < 0.0;
	let z = v[2] + v[3] < 0.0 || -v[2] + v[3] < 0.0;
	x || y || z
}

pub fn screen_space(ndc: Vector<f32, 3>, width: f32, height: f32) -> Vector<f32, 3> {
	vector![
		(ndc[0] + 1.0) / 2.0 * width,
		(1.0 - ndc[1]) / 2.0 * height,
		-ndc[2],
	]
}

pub fn edge<T: matrix::Cell>(a: Vector<T, 2>, b: Vector<T, 2>, p: Vector<T, 2>) -> T {
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

pub fn triangle(
	p1: Vector<f32, 3>,
	p2: Vector<f32, 3>,
	p3: Vector<f32, 3>,
	width: usize,
	height: usize,
	mut pixel: impl FnMut(usize, usize, f32, f32, f32),
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

pub fn blinn_phong(
	position: Vector<f32, 3>,
	normal: Vector<f32, 3>,
	uvs: Option<Vector<f32, 2>>,
	camera: Vector<f32, 3>,
	lights: &[Light],
	material: &obj::Material,
) -> Array<f32, 3> {
	let camera_dir = (camera - position).normalize();

	lights.iter().fold(array![0.0; 3], |sum, light| {
		let light_dir = (light.position - position).normalize();

		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let mapped_diffuse = uvs
			.map(|uv| material.diffuse(uv))
			.unwrap_or(material.diffuse);

		let halfway_vector = (light_dir + camera_dir).normalize();
		let specular = normal
			.dot(halfway_vector)
			.powi(material.specular_exponent as i32);
		let mapped_specular = uvs
			.map(|uv| material.specular(uv))
			.unwrap_or(material.specular);

		sum + mapped_diffuse * diffuse * light.diffuse_color
			+ mapped_specular * specular * light.specular_color
	}) * 255.0
}
