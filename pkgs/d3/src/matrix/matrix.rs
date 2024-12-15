use num::traits::{FromPrimitive, Num, NumAssignOps};
use std::ops::Index;
use std::ops::Mul;

pub trait Cell: Copy + Num + PartialOrd + FromPrimitive + NumAssignOps {}

impl Cell for f32 {}
impl Cell for i32 {}

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
		Matrix::new([[T::zero(); C]; R])
	}

	pub fn transpose(&self) -> Matrix<T, C, R> {
		let mut m = Matrix::zero();

		for r in 0..R {
			for c in 0..C {
				m[(c, r)] = self[(r, c)];
			}
		}

		return m;
	}
}

impl<T: Cell, const R: usize, const C: usize> Index<(usize, usize)> for Matrix<T, R, C> {
	type Output = T;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.0[index.0][index.1]
	}
}

impl<T: Cell, const R: usize, const C: usize> std::ops::IndexMut<(usize, usize)>
	for Matrix<T, R, C>
{
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.0[index.0][index.1]
	}
}
impl<T: Cell, const R: usize, const C: usize, const K: usize> Mul<Matrix<T, C, K>>
	for Matrix<T, R, C>
{
	type Output = Matrix<T, R, K>;

	fn mul(self, other: Matrix<T, C, K>) -> Self::Output {
		let mut m = Matrix::zero();

		for r in 0..R {
			for c in 0..K {
				for n in 0..C {
					m[(r, c)] += self[(r, n)] * other[(n, c)];
				}
			}
		}

		return m;
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::matrix::Matrix;

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
