use array::{Array, array};
use matrix::{Matrix, Vector, vector};

use crate::light::Light;

pub fn screen_space2(v: f32, scale: f32, bias: f32) -> usize {
	(scale * (v + 1.0) / 2.0 + bias).clamp(0.0, scale) as usize
}

pub fn bounding_box2(vs: [Vector<f32, 4>; 3]) -> Option<(f32, f32, f32, f32)> {
	let mut left = 1.0;
	let mut right = -1.0;
	let mut bottom = 1.0;
	let mut top = -1.0;

	let mut any_visible = false;
	let mut outcodes = [0u32; 3];
	let mut ocumulate = 0u32;
	let mut acumulate = !0u32;

	for (i, v) in vs.iter().enumerate() {
		let mut out = 0u32;

		if v[0] < -v[3] {
			out |= 0x01;
		}

		if v[0] > v[3] {
			out |= 0x02;
		}

		if v[1] < -v[3] {
			out |= 0x04;
		}

		if v[1] > v[3] {
			out |= 0x08;
		}

		if v[2] < 0.0 {
			out |= 0x10;
		}

		if v[2] > v[3] {
			out |= 0x20;
		}

		outcodes[i] = out;
		ocumulate |= out;
		acumulate &= out;

		if out & 0x03 == 0 {
			if v[0] - left * v[3] < 0.0 {
				left = v[0] / v[3];
			}

			if v[0] - right * v[3] > 0.0 {
				right = v[0] / v[3];
			}
		}

		if out & 0x0c == 0 {
			if v[1] - bottom * v[3] < 0.0 {
				bottom = v[1] / v[3];
			}

			if v[1] - top * v[3] > 0.0 {
				top = v[1] / v[3];
			}
		}

		if out == 0 {
			any_visible = true;
		}
	}

	if ocumulate == 0 {
		return Some((left, right, bottom, top));
	} else if acumulate != 0 {
		return None;
	} else if !any_visible {
		return Some((-1.0, 1.0, -1.0, 1.0));
	}

	for (i, v) in vs.iter().enumerate() {
		if (outcodes[i] & 0x01 != 0) && v[0] - left * v[3] < 0.0 {
			left = -1.0;
		};

		if (outcodes[i] & 0x02 != 0) && v[0] - right * v[3] > 0.0 {
			right = 1.0;
		};

		if (outcodes[i] & 0x04 != 0) && v[1] - bottom * v[3] < 0.0 {
			bottom = -1.0;
		};

		if (outcodes[i] & 0x08 != 0) && v[1] - top * v[3] > 0.0 {
			top = 1.0;
		};
	}

	Some((left, right, bottom, top))
}

pub fn inside(e: Vector<f32, 3>, p: Vector<f32, 3>) -> Option<f32> {
	let v = e.dot(p);

	if v > 0.0 {
		return Some(v);
	} else if v < 0.0 {
		return None;
	}

	if e[0] > 0.0 {
		return Some(v);
	} else if e[0] < 0.0 {
		return None;
	}

	if e[0] == 0.0 && e[1] < 0.0 {
		return None;
	}

	Some(v)
}

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

pub fn triangle(
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

#[allow(dead_code)]
pub fn fragments(
	p1: Vector<f32, 3>,
	p2: Vector<f32, 3>,
	p3: Vector<f32, 3>,
	width: usize,
	height: usize,
) -> impl Iterator<Item = (usize, usize, f32, f32, f32)> {
	let (min_x, min_y, max_x, max_y) = bounding_box((p1, p2, p3), (width, height));
	let area = 1.0 / edge(p1, p2, p3);
	let point = vector![min_x as f32, min_y as f32, 0.0];

	let mut r1 = edge(p2, p3, point);
	let mut r2 = edge(p3, p1, point);
	let mut r3 = edge(p1, p2, point);

	std::iter::from_coroutine(
		#[coroutine]
		move || {
			for y in min_y..=max_y {
				let mut u = r1;
				let mut v = r2;
				let mut w = r3;

				for x in min_x..=max_x {
					if u >= 0.0 && v >= 0.0 && w >= 0.0 {
						let u = u * area;
						let v = v * area;
						let w = w * area;

						yield (x, y, u, v, w);
					}

					u += p3[1] - p2[1];
					v += p1[1] - p3[1];
					w += p2[1] - p1[1];
				}

				r1 += p2[0] - p3[0];
				r2 += p3[0] - p1[0];
				r3 += p1[0] - p2[0];
			}
		},
	)
}

pub fn blinn_phong(
	position: Vector<f32, 3>,
	normal: Vector<f32, 3>,
	uv: Option<Vector<f32, 2>>,
	camera: Vector<f32, 3>,
	lights: &[Light],
	material: &obj::Material,
) -> Array<f32, 3> {
	let camera_dir = (camera - position).normalize();

	let color = (lights.iter()).fold(material.emissive(uv), |sum, light| {
		let light_dir = (light.position - position).normalize();

		let diffuse = light_dir.dot(normal).clamp(0.0, 1.0);
		let halfway_vector = (light_dir + camera_dir).normalize();
		let specular = normal
			.dot(halfway_vector)
			.powi(material.specular_exponent(uv) as i32);

		sum + material.diffuse(uv) * diffuse * light.diffuse_color
			+ material.specular(uv) * specular * light.specular_color
	});

	(color * 255.0).clamp(0.0, 255.0)
}

pub fn inverse(m: Matrix<f32, 3, 3>) -> Option<Matrix<f32, 3, 3>> {
	let a = m[(1, 1)] * m[(2, 2)] - m[(1, 2)] * m[(2, 1)];
	let b = m[(1, 2)] * m[(2, 0)] - m[(1, 0)] * m[(2, 2)];
	let c = m[(1, 0)] * m[(2, 1)] - m[(1, 1)] * m[(2, 0)];

	let det = m[(0, 0)] * a + m[(0, 1)] * b + m[(0, 2)] * c;
	if det <= 0.0 {
		return None;
	}

	let d = m[(0, 2)] * m[(2, 1)] - m[(0, 1)] * m[(2, 2)];
	let e = m[(0, 0)] * m[(2, 2)] - m[(0, 2)] * m[(2, 0)];
	let f = m[(0, 1)] * m[(2, 0)] - m[(0, 0)] * m[(2, 1)];
	let g = m[(0, 1)] * m[(1, 2)] - m[(0, 2)] * m[(1, 1)];
	let h = m[(0, 2)] * m[(1, 0)] - m[(0, 0)] * m[(1, 2)];
	let i = m[(0, 0)] * m[(1, 1)] - m[(0, 1)] * m[(1, 0)];

	let mut m = Matrix::zero();

	let s = 1.0 / det;

	m[(0, 0)] = s * a;
	m[(0, 1)] = s * d;
	m[(0, 2)] = s * g;
	m[(1, 0)] = s * b;
	m[(1, 1)] = s * e;
	m[(1, 2)] = s * h;
	m[(2, 0)] = s * c;
	m[(2, 1)] = s * f;
	m[(2, 2)] = s * i;

	Some(m)
}
