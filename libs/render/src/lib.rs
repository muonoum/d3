#![feature(iter_array_chunks)]

pub mod bounds;
pub mod buffer;
pub mod light;
pub mod pipeline;
pub mod texture;
pub mod varying;

pub use bounds::Bounds;
pub use pipeline::Pipeline;

use matrix::{Matrix, Vector};

#[inline]
pub fn inside(e: Vector<f32, 3>, v: Vector<f32, 3>) -> Option<f32> {
	let e = e.dot(v);
	if e > 0.0 { Some(e) } else { None }
}

#[inline]
pub fn adjugate(
	v1: Vector<f32, 4>,
	v2: Vector<f32, 4>,
	v3: Vector<f32, 4>,
) -> Option<Matrix<f32, 3, 3>> {
	let m13 = v3[0] * v2[1] - v2[0] * v3[1];
	let m23 = v1[0] * v3[1] - v3[0] * v1[1];
	let m33 = v2[0] * v1[1] - v1[0] * v2[1];

	if m13 * v1[3] + m23 * v2[3] + m33 * v3[3] <= 0.0 {
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
