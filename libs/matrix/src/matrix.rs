use num::traits::{FromPrimitive, Num, NumAssignOps};
use std::ops::{Div, Index, IndexMut, Mul};

use crate::{Vector, vector};

pub trait Cell = Copy + Num + PartialOrd + FromPrimitive + NumAssignOps;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<T: Cell, const R: usize, const C: usize>([[T; C]; R]);

impl<T: Cell, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn new(cells: [[T; C]; R]) -> Self {
		Self(cells)
	}

	pub fn row_vectors(&self) -> [Vector<T, C>; R] {
		self.0.map(|row| Vector::new([row]))
	}

	pub fn from_row_vectors(vs: [crate::Vector<T, C>; R]) -> Matrix<T, R, C> {
		Self::from_fn(|row, column| vs[row][column])
	}

	pub fn from_column_vectors(vs: [crate::Vector<T, R>; C]) -> Matrix<T, R, C> {
		Self::from_fn(|row, column| vs[column][row])
	}

	pub fn from_fn(f: impl Fn(usize, usize) -> T) -> Self {
		Self(std::array::from_fn(|row| {
			std::array::from_fn(|column| f(row, column))
		}))
	}

	pub fn zero() -> Self {
		Self([[T::zero(); C]; R])
	}

	pub fn transpose(&self) -> Matrix<T, C, R> {
		let mut matrix = Matrix::zero();

		for r in 0..R {
			for c in 0..C {
				matrix[(c, r)] = self[(r, c)];
			}
		}

		matrix
	}
}

impl<T: Cell, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.0[index.0][index.1]
	}
}

impl<T: Cell, const R: usize, const C: usize> IndexMut<(usize, usize)> for Matrix<T, R, C> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.0[index.0][index.1]
	}
}

impl<T: Cell, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>>
	for Matrix<T, R, C>
{
	type Output = Matrix<T, R, K>;

	fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
		let mut matrix = Matrix::zero();

		for r in 0..R {
			for c in 0..K {
				for n in 0..C {
					matrix[(r, c)] += self[(r, n)] * other[(n, c)];
				}
			}
		}

		matrix
	}
}

impl<T: Cell, const R: usize, const C: usize> Mul<T> for Matrix<T, R, C> {
	type Output = Self;

	fn mul(self, other: T) -> Self {
		Self::from_fn(|row, column| self[(row, column)] * other)
	}
}

impl<T: Cell, const R: usize, const C: usize> Div<T> for Matrix<T, R, C> {
	type Output = Self;

	fn div(self, other: T) -> Self {
		Self::from_fn(|row, column| self[(row, column)] / other)
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;

	#[test]
	fn zero_test() {
		let want = Matrix::new([[0.0, 0.0], [0.0, 0.0]]);
		let have = Matrix::zero();
		assert_eq!(want, have);
	}

	#[test]
	fn identity_test() {
		let want = Matrix::new([[1.0, 0.0], [0.0, 1.0]]);
		let have = Matrix::identity();
		assert_eq!(want, have);
	}

	#[test]
	fn transpose_test() {
		#[rustfmt::skip]
		let input = Matrix::new([
			[11.0, 12.0, 13.0, 14.0],
			[21.0, 22.0, 23.0, 24.0],
			[31.0, 32.0, 33.0, 34.0],
			[41.0, 42.0, 43.0, 44.0],
		]);

		#[rustfmt::skip]
		let want = Matrix::new([
			[11.0, 21.0, 31.0, 41.0],
			[12.0, 22.0, 32.0, 42.0],
			[13.0, 23.0, 33.0, 43.0],
			[14.0, 24.0, 34.0, 44.0],
		]);

		assert_eq!(input.transpose(), want);

		#[rustfmt::skip]
		let input = Matrix::new([
			[2.0, -9.0, 3.0],
			[13.0, 11.0, -17.0],
			[3.0, 6.0, 15.0],
			[4.0, 13.0, 1.0]
		]);

		#[rustfmt::skip]
		let want = Matrix::new([
			[2.0, 13.0, 3.0, 4.0],
			[-9.0, 11.0, 6.0, 13.0],
			[3.0, -17.0, 15.0, 1.0]
		]);

		assert_eq!(input.transpose(), want);
	}

	#[test]
	fn from_vectors_test() {
		let v1 = crate::vector![1.1, 1.2, 1.3];
		let v2 = crate::vector![2.1, 2.2, 2.3];
		let v3 = crate::vector![3.1, 3.2, 3.3];

		#[rustfmt::skip]
		let want1 = Matrix::new([
			[1.1, 1.2, 1.3],
			[2.1, 2.2, 2.3],
			[3.1, 3.2, 3.3]
		]);

		let have1 = Matrix::from_row_vectors([v1, v2, v3]);
		assert_eq!(want1, have1);

		#[rustfmt::skip]
		let want2 = Matrix::new([
			[1.1, 2.1, 3.1],
			[1.2, 2.2, 3.2],
			[1.3, 2.3, 3.3]
		]);

		let have2 = Matrix::from_column_vectors([v1, v2, v3]);
		assert_eq!(want2, have2);
	}
}
