use array::Array;
use matrix::Vector;

pub trait Varying {
	fn scale(self, rz: f32) -> Self;
	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self;
}

impl<const D: usize> Varying for Array<f32, D> {
	fn scale(self, rz: f32) -> Self {
		self * rz
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a * u + b * v + c * w
	}
}

impl<const D: usize> Varying for Vector<f32, D> {
	fn scale(self, rz: f32) -> Self {
		self * rz
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a * u + b * v + c * w
	}
}

impl<T: Varying> Varying for Option<T> {
	fn scale(self, rz: f32) -> Self {
		self.map(|v| v.scale(rz))
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		a.and_then(|a| b.and_then(|b| c.map(|c| T::barycentric(a, u, b, v, c, w))))
	}
}

impl Varying for () {
	fn scale(self, _rz: f32) -> Self {}
	fn barycentric(_a: Self, _u: f32, _b: Self, _v: f32, _c: Self, _w: f32) -> Self {}
}

impl<A, B> Varying for (A, B)
where
	A: Varying,
	B: Varying,
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

impl<A, B, C> Varying for (A, B, C)
where
	A: Varying,
	B: Varying,
	C: Varying,
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

impl<A, B, C, D> Varying for (A, B, C, D)
where
	A: Varying,
	B: Varying,
	C: Varying,
	D: Varying,
{
	fn scale(self, rz: f32) -> Self {
		(
			self.0.scale(rz),
			self.1.scale(rz),
			self.2.scale(rz),
			self.3.scale(rz),
		)
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = A::barycentric(a.0, u, b.0, v, c.0, w);
		let r1 = B::barycentric(a.1, u, b.1, v, c.1, w);
		let r2 = C::barycentric(a.2, u, b.2, v, c.2, w);
		let r3 = D::barycentric(a.3, u, b.3, v, c.3, w);
		(r0, r1, r2, r3)
	}
}

impl<A, B, C, D, E> Varying for (A, B, C, D, E)
where
	A: Varying,
	B: Varying,
	C: Varying,
	D: Varying,
	E: Varying,
{
	fn scale(self, rz: f32) -> Self {
		(
			self.0.scale(rz),
			self.1.scale(rz),
			self.2.scale(rz),
			self.3.scale(rz),
			self.4.scale(rz),
		)
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = A::barycentric(a.0, u, b.0, v, c.0, w);
		let r1 = B::barycentric(a.1, u, b.1, v, c.1, w);
		let r2 = C::barycentric(a.2, u, b.2, v, c.2, w);
		let r3 = D::barycentric(a.3, u, b.3, v, c.3, w);
		let r4 = E::barycentric(a.4, u, b.4, v, c.4, w);
		(r0, r1, r2, r3, r4)
	}
}

impl<A, B, C, D, E, F> Varying for (A, B, C, D, E, F)
where
	A: Varying,
	B: Varying,
	C: Varying,
	D: Varying,
	E: Varying,
	F: Varying,
{
	fn scale(self, rz: f32) -> Self {
		(
			self.0.scale(rz),
			self.1.scale(rz),
			self.2.scale(rz),
			self.3.scale(rz),
			self.4.scale(rz),
			self.5.scale(rz),
		)
	}

	fn barycentric(a: Self, u: f32, b: Self, v: f32, c: Self, w: f32) -> Self {
		let r0 = A::barycentric(a.0, u, b.0, v, c.0, w);
		let r1 = B::barycentric(a.1, u, b.1, v, c.1, w);
		let r2 = C::barycentric(a.2, u, b.2, v, c.2, w);
		let r3 = D::barycentric(a.3, u, b.3, v, c.3, w);
		let r4 = E::barycentric(a.4, u, b.4, v, c.4, w);
		let r5 = F::barycentric(a.5, u, b.5, v, c.5, w);
		(r0, r1, r2, r3, r4, r5)
	}
}
