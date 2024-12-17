use matrix::matrix::Matrix;
use matrix::vector::Vector;

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
	Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, a.cos(), a.sin(), 0.0],
		[0.0, -a.sin(), a.cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_y(a: f32) -> Matrix<f32, 4, 4> {
	Matrix::new([
		[a.cos(), 0.0, -a.sin(), 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[a.sin(), 0.0, a.cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn rotate_z(a: f32) -> Matrix<f32, 4, 4> {
	Matrix::new([
		[a.cos(), 0.0, -a.sin(), 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[a.sin(), 0.0, a.cos(), 0.0],
		[0.0, 0.0, 0.0, 1.0],
	])
}

pub fn look_at(
	from: Vector<f32, 3>,
	to: Vector<f32, 3>,
	up: Vector<f32, 3>,
) -> Matrix<f32, 4, 4> {
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

pub fn perspective_near(aspect: f32, fov: f32, near: f32) -> Matrix<f32, 4, 4> {
	Matrix::new([
		[fov / aspect, 0.0, 0.0, 0.0],
		[0.0, fov, 0.0, 0.0],
		[0.0, 0.0, 0.0, -1.0],
		[0.0, 0.0, -near, 0.0],
	])
}

pub fn perspective_near_far(aspect: f32, fov: f32, near: f32, far: f32) -> Matrix<f32, 4, 4> {
	let sy = 1.0 / (fov / 2.0).tan();
	let sx = sy / aspect;
	let nmf = near - far;

	Matrix::new([
		[sx, 0.0, 0.0, 0.0],
		[0.0, sy, 0.0, 0.0],
		[0.0, 0.0, (far + near) / nmf, -1.0],
		[0.0, 0.0, 2.0 * near * far / nmf, 0.0],
	])
}

pub fn viewport(width: f32, height: f32) -> Matrix<f32, 4, 4> {
	let flip = Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, -1.0, 0.0, 0.0],
		[0.0, 0.0, -1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);

	let translate = Matrix::new([
		[1.0, 0.0, 0.0, 0.0],
		[0.0, 1.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[1.0, 1.0, 0.0, 1.0],
	]);

	let scale = Matrix::new([
		[width / 2.0, 0.0, 0.0, 0.0],
		[0.0, height / 2.0, 0.0, 0.0],
		[0.0, 0.0, 1.0, 0.0],
		[0.0, 0.0, 0.0, 1.0],
	]);

	flip * translate * scale
}
