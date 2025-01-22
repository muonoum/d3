use matrix::{Matrix, Vector};

pub fn scale_vector(v: Vector<f32, 3>) -> Matrix<f32, 4, 4> {
	scale(v[0], v[1], v[2])
}

pub fn scale(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
	Matrix::new([
		[x, 0.0, 0.0, 0.0],
		[0.0, y, 0.0, 0.0],
		[0.0, 0.0, z, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn translate_vector(v: Vector<f32, 3>) -> Matrix<f32, 4, 4> {
	translate(v[0], v[1], v[2])
}

pub fn translate(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
	Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[x, y, z, 1.0],
	])
}

pub fn rotate(x: f32, y: f32, z: f32) -> Matrix<f32, 4, 4> {
	rotate_z(z) * rotate_y(y) * rotate_x(x)
}

pub fn rotate_vector(v: Vector<f32, 3>) -> Matrix<f32, 4, 4> {
	rotate(v[0], v[1], v[2])
}

pub fn rotate_x(a: f32) -> Matrix<f32, 4, 4> {
	let (sin, cos) = a.sin_cos();

	Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, cos, sin, 0.0],
		[0.0, -sin, cos, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_y(a: f32) -> Matrix<f32, 4, 4> {
	let (sin, cos) = a.sin_cos();

	Matrix::new([
		[cos, 0.0, -sin, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[sin, 0.0, cos, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_z(a: f32) -> Matrix<f32, 4, 4> {
	let (sin, cos) = a.sin_cos();

	Matrix::new([
		[cos, sin, 0.0, 0.0],
		[-sin, cos, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn look_to(from: Vector<f32, 3>, to: Vector<f32, 3>, up: Vector<f32, 3>) -> Matrix<f32, 4, 4> {
	let forward = (to - from).normalize();
	let right = forward.cross(up).normalize();
	let up = right.cross(forward);

	Matrix::new([
		[right[0], up[0], -forward[0], 0.0],
		[right[1], up[1], -forward[1], 0.0],
		[right[2], up[2], -forward[2], 0.0],
		[-from.dot(right), -from.dot(up), from.dot(forward), 1.0],
	])
}

pub fn look_at(from: Vector<f32, 3>, to: Vector<f32, 3>, up: Vector<f32, 3>) -> Matrix<f32, 4, 4> {
	let forward = (from - to).normalize();
	let right = up.cross(forward).normalize();
	let up = forward.cross(right);

	Matrix::new([
		[right[0], right[1], right[2], 0.0],
		[up[0], up[1], up[2], 0.0],
		[forward[0], forward[1], forward[2], 0.0],
		[from[0], from[1], from[2], 1.0],
	])
}

pub fn perspective(ratio: f32, fov_y: f32) -> Matrix<f32, 4, 4> {
	let fov = (fov_y / 2.0).tan().recip();

	Matrix::new([
		[fov / ratio, 0.0, 0.0, 0.0],
		[0.0, fov, 0.0, 0.0],
		[0.0, 0.0, -1.0, -1.0],
		[0.0, 0.0, 0.0, 0.0],
	])
}

pub fn perspective_near(ratio: f32, fov_y: f32, near: f32) -> Matrix<f32, 4, 4> {
	let mut m = perspective(ratio, fov_y);
	m[(3, 2)] = -near;
	m
}

pub fn perspective_near_far(ratio: f32, fov_y: f32, near: f32, far: f32) -> Matrix<f32, 4, 4> {
	let mut m = perspective_near(ratio, fov_y, near);
	m[(2, 2)] = (far + near) / (near - far);
	m[(3, 2)] = (2.0 * far * near) / (near - far);
	m
}
