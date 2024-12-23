use array::Array;
use matrix::Vector;

pub trait Interpolate {
	fn scale(self, rz: f32) -> Self;
	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self;
}

impl<T: Interpolate> Interpolate for Option<T> {
	fn scale(self, rz: f32) -> Self {
		self.map(|v| v.scale(rz))
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		match (a, b, c) {
			(Some(a), Some(b), Some(c)) => Some(T::barycentric(a, u, b, v, c, w)),
			_else => None,
		}
	}
}

impl Interpolate for Array<f32, 3> {
	fn scale(self, rz: f32) -> Self {
		self * rz
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a * u + b * v + c * w
	}
}

impl Interpolate for Vector<f32, 2> {
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
	A: Interpolate,
	B: Interpolate,
{
	fn scale(self, rz: f32) -> Self {
		(self.0.scale(rz), self.1.scale(rz))
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = A::barycentric(a.0, u, b.0, v, c.0, w);
		let r1 = B::barycentric(a.1, u, b.1, v, c.1, w);
		(r0, r1)
	}
}

impl<A, B, C> Interpolate for (A, B, C)
where
	A: Interpolate,
	B: Interpolate,
	C: Interpolate,
{
	fn scale(self, rz: f32) -> Self {
		(self.0.scale(rz), self.1.scale(rz), self.2.scale(rz))
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = A::barycentric(a.0, u, b.0, v, c.0, w);
		let r1 = B::barycentric(a.1, u, b.1, v, c.1, w);
		let r2 = C::barycentric(a.2, u, b.2, v, c.2, w);
		(r0, r1, r2)
	}
}
