use num::traits::{FromPrimitive, Num, NumAssignOps};
use std::ops::{Div, Index, IndexMut, Mul};

pub trait Cell = Copy + Num + PartialOrd + FromPrimitive + NumAssignOps;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<T: Cell, const R: usize, const C: usize>([[T; C]; R]);

impl<T: Cell, const R: usize, const C: usize> Matrix<T, R, C> {
	pub fn new(cells: [[T; C]; R]) -> Self {
		Self(cells)
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
		let input = Matrix::new([
			[11.0, 12.0, 13.0, 14.0],
			[21.0, 22.0, 23.0, 24.0],
			[31.0, 32.0, 33.0, 34.0],
			[41.0, 42.0, 43.0, 44.0],
		]);

		let want = Matrix::new([
			[11.0, 21.0, 31.0, 41.0],
			[12.0, 22.0, 32.0, 42.0],
			[13.0, 23.0, 33.0, 43.0],
			[14.0, 24.0, 34.0, 44.0],
		]);

		assert_eq!(input.transpose(), want);

		let input = Matrix::new([
			[2.0, -9.0, 3.0],
			[13.0, 11.0, -17.0],
			[3.0, 6.0, 15.0],
			[4.0, 13.0, 1.0],
		]);

		let want = Matrix::new([
			[2.0, 13.0, 3.0, 4.0],
			[-9.0, 11.0, 6.0, 13.0],
			[3.0, -17.0, 15.0, 1.0],
		]);

		assert_eq!(input.transpose(), want);
	}
}
