use crate::matrix::Cell;
use crate::matrix::Matrix;

// impl<T: Cell, const R: usize, const C: usize> Matrix<T, R, C> {
// 	pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<T, { R - 1 }, { C - 1 }>, ()> {
// 		let mut m = Matrix::zero();
// 		let mut next_column = 0;
// 		let mut next_row = 0;

// 		for r in 0..2 {
// 			if r == row {
// 				continue;
// 			}

// 			for c in 0..2 {
// 				if c == col {
// 					continue;
// 				}

// 				m[[next_row, next_column]] = self[[r, c]];
// 				next_column += 1;
// 			}

// 			next_column = 0;
// 			next_row += 1;
// 		}

// 		return Ok(m);
// 	}
// }

impl<T: Cell> Matrix<T, 2, 2> {
	#[allow(clippy::result_unit_err)]
	pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<T, 1, 1>, ()> {
		match (row, col) {
			(0, 0) => Ok(Matrix::new([[self[(1, 1)]]])),
			(0, 1) => Ok(Matrix::new([[self[(1, 0)]]])),
			(1, 0) => Ok(Matrix::new([[self[(0, 1)]]])),
			(1, 1) => Ok(Matrix::new([[self[(0, 0)]]])),
			_ => Err(()),
		}
	}
}

impl<T: Cell> Matrix<T, 3, 3> {
	#[allow(clippy::result_unit_err)]
	pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<T, 2, 2>, ()> {
		match (row, col) {
			(0, 0) => Ok(Matrix::new([
				[self[(1, 1)], self[(1, 2)]],
				[self[(2, 1)], self[(2, 2)]],
			])),

			(0, 1) => Ok(Matrix::new([
				[self[(1, 0)], self[(1, 2)]],
				[self[(2, 0)], self[(2, 2)]],
			])),

			(0, 2) => Ok(Matrix::new([
				[self[(1, 0)], self[(1, 1)]],
				[self[(2, 0)], self[(2, 1)]],
			])),

			(1, 0) => Ok(Matrix::new([
				[self[(0, 1)], self[(0, 2)]],
				[self[(2, 1)], self[(2, 2)]],
			])),

			(1, 1) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 2)]],
				[self[(2, 0)], self[(2, 2)]],
			])),

			(1, 2) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)]],
				[self[(2, 0)], self[(2, 1)]],
			])),

			(2, 0) => Ok(Matrix::new([
				[self[(0, 1)], self[(0, 2)]],
				[self[(1, 1)], self[(1, 2)]],
			])),

			(2, 1) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 2)]],
				[self[(1, 0)], self[(1, 2)]],
			])),

			(2, 2) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)]],
				[self[(1, 0)], self[(1, 1)]],
			])),

			_ => Err(()),
		}
	}
}

impl<T: Cell> Matrix<T, 4, 4> {
	#[allow(clippy::result_unit_err)]
	pub fn sub_matrix(self, row: usize, col: usize) -> Result<Matrix<T, 3, 3>, ()> {
		match (row, col) {
			(0, 0) => Ok(Matrix::new([
				[self[(1, 1)], self[(1, 2)], self[(1, 3)]],
				[self[(2, 1)], self[(2, 2)], self[(2, 3)]],
				[self[(3, 1)], self[(3, 2)], self[(3, 3)]],
			])),

			(0, 1) => Ok(Matrix::new([
				[self[(1, 0)], self[(1, 2)], self[(1, 3)]],
				[self[(2, 0)], self[(2, 2)], self[(2, 3)]],
				[self[(3, 0)], self[(3, 2)], self[(3, 3)]],
			])),

			(0, 2) => Ok(Matrix::new([
				[self[(1, 0)], self[(1, 1)], self[(1, 3)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 3)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 3)]],
			])),

			(0, 3) => Ok(Matrix::new([
				[self[(1, 0)], self[(1, 1)], self[(1, 2)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 2)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 2)]],
			])),

			(1, 0) => Ok(Matrix::new([
				[self[(0, 1)], self[(0, 2)], self[(0, 3)]],
				[self[(2, 1)], self[(2, 2)], self[(2, 3)]],
				[self[(3, 1)], self[(3, 2)], self[(3, 3)]],
			])),

			(1, 1) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 2)], self[(0, 3)]],
				[self[(2, 0)], self[(2, 2)], self[(2, 3)]],
				[self[(3, 0)], self[(3, 2)], self[(3, 3)]],
			])),

			(1, 2) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 3)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 3)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 3)]],
			])),

			(1, 3) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 2)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 2)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 2)]],
			])),

			(2, 0) => Ok(Matrix::new([
				[self[(0, 1)], self[(0, 2)], self[(0, 3)]],
				[self[(1, 1)], self[(1, 2)], self[(1, 3)]],
				[self[(3, 1)], self[(3, 2)], self[(3, 3)]],
			])),

			(2, 1) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 2)], self[(0, 3)]],
				[self[(1, 0)], self[(1, 2)], self[(1, 3)]],
				[self[(3, 0)], self[(3, 2)], self[(3, 3)]],
			])),

			(2, 2) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 3)]],
				[self[(1, 0)], self[(1, 1)], self[(1, 3)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 3)]],
			])),

			(2, 3) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 2)]],
				[self[(1, 0)], self[(1, 1)], self[(1, 2)]],
				[self[(3, 0)], self[(3, 1)], self[(3, 2)]],
			])),

			(3, 0) => Ok(Matrix::new([
				[self[(0, 1)], self[(0, 2)], self[(0, 3)]],
				[self[(1, 1)], self[(1, 2)], self[(1, 3)]],
				[self[(2, 1)], self[(2, 2)], self[(2, 3)]],
			])),

			(3, 1) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 2)], self[(0, 3)]],
				[self[(1, 0)], self[(1, 2)], self[(1, 3)]],
				[self[(2, 0)], self[(2, 2)], self[(2, 3)]],
			])),

			(3, 2) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 3)]],
				[self[(1, 0)], self[(1, 1)], self[(1, 3)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 3)]],
			])),

			(3, 3) => Ok(Matrix::new([
				[self[(0, 0)], self[(0, 1)], self[(0, 2)]],
				[self[(1, 0)], self[(1, 1)], self[(1, 2)]],
				[self[(2, 0)], self[(2, 1)], self[(2, 2)]],
			])),

			_ => Err(()),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::matrix::Matrix;

	#[test]
	fn sub2_test() {
		let input = Matrix::new([[11.0, 12.0], [21.0, 22.0]]);
		assert_eq!(input.sub_matrix(0, 0), Ok(Matrix::new([[22.0]])));
		assert_eq!(input.sub_matrix(0, 1), Ok(Matrix::new([[21.0]])));
		assert_eq!(input.sub_matrix(1, 0), Ok(Matrix::new([[12.0]])));
		assert_eq!(input.sub_matrix(1, 1), Ok(Matrix::new([[11.0]])));
	}
	#[test]
	fn sub3_test() {
		let input = Matrix::new([[11.0, 12.0, 13.0], [21.0, 22.0, 23.0], [31.0, 32.0, 33.0]]);

		assert_eq!(
			input.sub_matrix(0, 0),
			Ok(Matrix::new([[22.0, 23.0], [32.0, 33.0]]))
		);

		assert_eq!(
			input.sub_matrix(0, 1),
			Ok(Matrix::new([[21.0, 23.0], [31.0, 33.0]]))
		);

		assert_eq!(
			input.sub_matrix(0, 2),
			Ok(Matrix::new([[21.0, 22.0], [31.0, 32.0]]))
		);

		assert_eq!(
			input.sub_matrix(1, 0),
			Ok(Matrix::new([[12.0, 13.0], [32.0, 33.0]]))
		);

		assert_eq!(
			input.sub_matrix(1, 1),
			Ok(Matrix::new([[11.0, 13.0], [31.0, 33.0]]))
		);

		assert_eq!(
			input.sub_matrix(1, 2),
			Ok(Matrix::new([[11.0, 12.0], [31.0, 32.0]]))
		);

		assert_eq!(
			input.sub_matrix(2, 0),
			Ok(Matrix::new([[12.0, 13.0], [22.0, 23.0]]))
		);

		assert_eq!(
			input.sub_matrix(2, 1),
			Ok(Matrix::new([[11.0, 13.0], [21.0, 23.0]]))
		);

		assert_eq!(
			input.sub_matrix(2, 2),
			Ok(Matrix::new([[11.0, 12.0], [21.0, 22.0]]))
		);
	}

	#[test]
	fn sub4_test() {
		let input = Matrix::new([
			[11.0, 12.0, 13.0, 14.0],
			[21.0, 22.0, 23.0, 24.0],
			[31.0, 32.0, 33.0, 34.0],
			[41.0, 42.0, 43.0, 44.0],
		]);

		assert_eq!(
			input.sub_matrix(0, 0),
			Ok(Matrix::new([
				[22.0, 23.0, 24.0],
				[32.0, 33.0, 34.0],
				[42.0, 43.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(0, 1),
			Ok(Matrix::new([
				[21.0, 23.0, 24.0],
				[31.0, 33.0, 34.0],
				[41.0, 43.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(0, 2),
			Ok(Matrix::new([
				[21.0, 22.0, 24.0],
				[31.0, 32.0, 34.0],
				[41.0, 42.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(0, 3),
			Ok(Matrix::new([
				[21.0, 22.0, 23.0],
				[31.0, 32.0, 33.0],
				[41.0, 42.0, 43.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(1, 0),
			Ok(Matrix::new([
				[12.0, 13.0, 14.0],
				[32.0, 33.0, 34.0],
				[42.0, 43.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(1, 1),
			Ok(Matrix::new([
				[11.0, 13.0, 14.0],
				[31.0, 33.0, 34.0],
				[41.0, 43.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(1, 2),
			Ok(Matrix::new([
				[11.0, 12.0, 14.0],
				[31.0, 32.0, 34.0],
				[41.0, 42.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(1, 3),
			Ok(Matrix::new([
				[11.0, 12.0, 13.0],
				[31.0, 32.0, 33.0],
				[41.0, 42.0, 43.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(2, 0),
			Ok(Matrix::new([
				[12.0, 13.0, 14.0],
				[22.0, 23.0, 24.0],
				[42.0, 43.0, 44.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(2, 1),
			Ok(Matrix::new([
				[11.0, 13.0, 14.0],
				[21.0, 23.0, 24.0],
				[41.0, 43.0, 44.0],
			]))
		);

		assert_eq!(
			input.sub_matrix(2, 2),
			Ok(Matrix::new([
				[11.0, 12.0, 14.0],
				[21.0, 22.0, 24.0],
				[41.0, 42.0, 44.0],
			]))
		);

		assert_eq!(
			input.sub_matrix(2, 3),
			Ok(Matrix::new([
				[11.0, 12.0, 13.0],
				[21.0, 22.0, 23.0],
				[41.0, 42.0, 43.0],
			]))
		);

		assert_eq!(
			input.sub_matrix(3, 0),
			Ok(Matrix::new([
				[12.0, 13.0, 14.0],
				[22.0, 23.0, 24.0],
				[32.0, 33.0, 34.0]
			]))
		);

		assert_eq!(
			input.sub_matrix(3, 1),
			Ok(Matrix::new([
				[11.0, 13.0, 14.0],
				[21.0, 23.0, 24.0],
				[31.0, 33.0, 34.0],
			]))
		);

		assert_eq!(
			input.sub_matrix(3, 2),
			Ok(Matrix::new([
				[11.0, 12.0, 14.0],
				[21.0, 22.0, 24.0],
				[31.0, 32.0, 34.0],
			]))
		);

		assert_eq!(
			input.sub_matrix(3, 3),
			Ok(Matrix::new([
				[11.0, 12.0, 13.0],
				[21.0, 22.0, 23.0],
				[31.0, 32.0, 33.0],
			]))
		);
	}
}
