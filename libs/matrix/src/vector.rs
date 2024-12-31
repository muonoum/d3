use crate::matrix::{Cell, Matrix};
use crate::vector;
use num::traits::Float;

#[macro_export]
macro_rules! vector {
	($repeat:expr; $n:expr) => {
		$crate::vector::Vector::new([[$repeat; $n]])
	};
	($($value:expr),* $(,)?) => {
		$crate::vector::Vector::new([[$($value), *]])
	};
}

pub type Vector<T, const D: usize> = Matrix<T, 1, D>;

impl<T: Cell + Float, const D: usize> Vector<T, D> {
	pub fn magnitude(self) -> T {
		self.dot(self).sqrt()
	}

	pub fn normalize(self) -> Self {
		self / self.magnitude()
	}
}

impl<T: Cell, const D: usize> Vector<T, D> {
	pub fn dot(self, other: Self) -> T {
		let mut product = T::zero();

		for i in 0..D {
			product += self[i] * other[i];
		}

		product
	}
}

impl<T: Cell> From<Vector<T, 3>> for Vector<T, 2> {
	fn from(vector: Vector<T, 3>) -> Vector<T, 2> {
		vector![vector[0], vector[1]]
	}
}

impl<T: Cell> Vector<T, 3> {
	pub fn v4(self) -> Vector<T, 4> {
		vector![self[0], self[1], self[2], T::one(),]
	}

	pub fn cross(self, other: Self) -> Self {
		vector![
			self[1] * other[2] - self[2] * other[1],
			self[2] * other[0] - self[0] * other[2],
			self[0] * other[1] - self[1] * other[0],
		]
	}
}

impl<T: Cell> Vector<T, 4> {
	pub fn v3(self) -> Vector<T, 3> {
		vector![self[0] / self[3], self[1] / self[3], self[2] / self[3],]
	}
}

impl<T: Cell, const D: usize> std::ops::Index<usize> for Vector<T, D> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		&self[(0, index)]
	}
}

impl<T: Cell, const D: usize> std::ops::IndexMut<usize> for Vector<T, D> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		&mut self[(0, index)]
	}
}

impl<T: Cell, const D: usize> std::ops::Add for Vector<T, D> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		Self::from_fn(|row, col| self[(row, col)] + other[(row, col)])
	}
}

impl<T: Cell, const D: usize> std::ops::Add<T> for Vector<T, D> {
	type Output = Self;

	fn add(self, other: T) -> Self {
		Self::from_fn(|row, col| self[(row, col)] + other)
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign<T> for Vector<T, D> {
	fn add_assign(&mut self, other: T) {
		for i in 0..D {
			self[(0, i)] += other;
		}
	}
}

impl<T: Cell, const D: usize> std::ops::AddAssign for Vector<T, D> {
	fn add_assign(&mut self, other: Self) {
		for i in 0..D {
			self[(0, i)] += other[(0, i)];
		}
	}
}

impl<T: Cell, const D: usize> std::ops::Sub for Vector<T, D> {
	type Output = Self;

	fn sub(self, other: Self) -> Self {
		Self::from_fn(|row, col| self[(row, col)] - other[(row, col)])
	}
}

impl<T: Cell, const D: usize> std::ops::Sub<T> for Vector<T, D> {
	type Output = Self;

	fn sub(self, other: T) -> Self {
		Self::from_fn(|row, col| self[(row, col)] - other)
	}
}

impl<const D: usize> std::ops::Neg for Vector<f32, D> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::from_fn(|row, col| -self[(row, col)])
	}
}
