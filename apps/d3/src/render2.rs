use matrix::{Matrix, Vector};

pub fn edge(f: Vector<f32, 3>, v: Vector<f32, 2>) -> f32 {
	v[0] * f[0] + v[1] * f[1] + f[2]
}

pub fn clipped(v1: Vector<f32, 4>, v2: Vector<f32, 4>, v3: Vector<f32, 4>) -> bool {
	if v1[0] < -v1[3] && v2[0] < -v2[3] && v3[0] < -v3[3] {
		return true; // left
	}

	if v1[0] > v1[3] && v2[0] > v2[3] && v3[0] > v3[3] {
		return true; // right
	}

	if v1[1] < -v1[3] && v2[1] < -v2[3] && v3[1] < -v3[3] {
		return true; // bottom
	}

	if v1[1] > v1[3] && v2[1] > v2[3] && v3[1] > v3[3] {
		return true; // top
	}

	if v1[2] < 0.0 && v2[2] < 0.0 && v3[2] < 0.0 {
		return true; // near
	}

	if v1[2] > v1[3] && v2[2] > v2[3] && v3[2] > v3[3] {
		return true; // far
	}

	// TODO
	let left = v1[0] >= -v1[3] && v2[0] >= -v2[3] && v3[0] >= -v3[3];
	let right = v1[0] <= v1[3] && v2[0] <= v2[3] && v3[0] <= v3[3];
	let bottom = v1[1] >= -v1[3] && v2[1] >= -v2[3] && v3[1] >= -v3[3];
	let top = v1[1] <= v1[3] && v2[1] <= v2[3] && v3[1] <= v3[3];
	let near = v1[2] >= 0.0 && v2[2] >= 0.0 && v3[2] >= 0.0;
	let far = v1[2] <= v1[3] && v2[2] <= v1[3] && v3[2] <= v1[3];
	if left || right || bottom || top || near || far {
		return false;
	}

	true
}

pub fn adjugate(
	v1: Vector<f32, 4>,
	v2: Vector<f32, 4>,
	v3: Vector<f32, 4>,
) -> Option<Matrix<f32, 3, 3>> {
	let m13 = v3[0] * v2[1] - v2[0] * v3[1];
	let m23 = v1[0] * v3[1] - v3[0] * v1[1];
	let m33 = v2[0] * v1[1] - v1[0] * v2[1];

	let det = m13 * v1[3] + m23 * v2[3] + m33 * v3[3];
	if det <= 0.0 {
		return None;
	}

	let m11 = v3[1] * v2[3] - v2[1] * v3[3];
	let m12 = v2[0] * v3[3] - v3[0] * v2[3];
	let m21 = v1[1] * v3[3] - v3[1] * v1[3];
	let m22 = v3[0] * v1[3] - v1[0] * v3[3];
	let m31 = v2[1] * v1[3] - v1[1] * v2[3];
	let m32 = v1[0] * v2[3] - v2[0] * v1[3];

	let r1 = [m11, m12, m13];
	let r2 = [m21, m22, m23];
	let r3 = [m31, m32, m33];
	Some(Matrix::new([r1, r2, r3]))
}

pub fn bounding_box(
	v1: Vector<f32, 4>,
	v2: Vector<f32, 4>,
	v3: Vector<f32, 4>,
	width: usize,
	height: usize,
) -> (usize, usize, usize, usize) {
	let v1 = v1.v3();
	let v2 = v2.v3();
	let v3 = v3.v3();

	let min_x = v1[0].min(v2[0]).min(v3[0]).max(0.0) as usize;
	let max_x = v1[0].max(v2[0]).max(v3[0]).min(width as f32 - 1.0) as usize;
	let min_y = v1[1].min(v2[1]).min(v3[1]).max(0.0) as usize;
	let max_y = v1[1].max(v2[1]).max(v3[1]).min(height as f32 - 1.0) as usize;

	(min_x, max_x, min_y, max_y)
}
