use array::Array;
use matrix::Vector;
use std::ops::{Add, Mul};

trait Interpolatable<T> = Copy + Mul<f32, Output = T> + Add<T, Output = T>;

pub trait Interpolate {
	fn scale(self, rz: f32) -> Self;
	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self;
}

impl Interpolate for Array<f32, 3> {
	fn scale(self, rz: f32) -> Self {
		self * rz
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a * u + b * v + c * w
	}
}

impl Interpolate for Vector<f32, 3> {
	fn scale(self, rz: f32) -> Self {
		self * rz
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a * u + b * v + c * w
	}
}

impl<A, B> Interpolate for (A, B)
where
	A: Interpolatable<A>,
	B: Interpolatable<B>,
{
	fn scale(self, rz: f32) -> Self {
		(self.0 * rz, self.1 * rz)
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let x = a.0 * u + b.0 * v + c.0 * w;
		let y = a.1 * u + b.1 * v + c.1 * w;
		(x, y)
	}
}

impl<A, B, C> Interpolate for (A, B, C)
where
	A: Interpolatable<A>,
	B: Interpolatable<B>,
	C: Interpolatable<C>,
{
	fn scale(self, rz: f32) -> Self {
		(self.0 * rz, self.1 * rz, self.2 * rz)
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = a.0 * u + b.0 * v + c.0 * w;
		let r1 = a.1 * u + b.1 * v + c.1 * w;
		let r2 = a.2 * u + b.2 * v + c.2 * w;
		(r0, r1, r2)
	}
}
